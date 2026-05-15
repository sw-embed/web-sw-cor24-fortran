//! Three-stage in-browser pipeline. Stage 1 chains dcftn's three
//! SNOBOL4-implemented compiler phases (no Rust-side Fortran parser):
//!
//!   user .f
//!     |   nested cor24-emu loads snobol4.lgo,
//!     |   loads normalize.sno @ 0xE0000 and .f @ 0xF0000,
//!     |   runs -> emits normalized statement records via UART
//!     v
//!   normalized records
//!     |   nested cor24-emu loads snobol4.lgo,
//!     |   loads classify.sno @ 0xE0000 and records @ 0xF0000,
//!     |   runs -> emits records with kind= field added
//!     v
//!   classified records
//!     |   nested cor24-emu loads snobol4.lgo,
//!     |   loads emit_asm.sno @ 0xE0000 and records @ 0xF0000,
//!     |   runs -> emits a complete .s including inlined runtime
//!     v
//!   .s assembly
//!     |   cor24-assembler -> bytes + listing  (stage 2)
//!     |   cor24-emulator -> UART output       (stage 3)
//!     v
//!   program output
//!
//! Each compiler phase mirrors `scripts/fortran` in `sw-cor24-fortran`.
//!
//! As of dcftn m13-inline-runtime, emit_asm.sno emits the full .s
//! including the runtime support routines (_start / _halt / _putc /
//! _puts / _putint / _aindex). The old `; __RUNTIME_PRELUDE__` /
//! `; __RUNTIME_PUTINT__` marker splicing is gone -- emit_asm fits
//! inside the SNOBOL4 source-buffer cap now that dcsno's
//! `pr/cap-and-pattern-fixes` bumped internal limits.
//!
//! Refreshing `assets/{normalize,classify,emit_asm}.sno` + the
//! `assets/snobol4.lgo` interpreter image is the only change needed
//! here as dcftn ships further milestones.

use cor24_assembler::{AssembledLine, Assembler};
use cor24_emulator::{EmulatorCore, StopReason};

pub const SNOBOL4_LGO: &str = include_str!("../assets/snobol4.lgo");
pub const NORMALIZE_SNO: &str = include_str!("../assets/normalize.sno");
pub const CLASSIFY_SNO: &str = include_str!("../assets/classify.sno");
pub const EMIT_ASM_SNO: &str = include_str!("../assets/emit_asm.sno");

const SNOBOL4_PHASE_BUDGET: u64 = 200_000_000;

// Post-2026-05-14 dcsno load-address convention (was 0x080000 / 0x090000
// pre `pr/cap-and-pattern-fixes`). The wrapper at `work/bin/snobol4`
// is a compat shim that rewrites the old addresses to the new ones at
// the CLI layer; we call EmulatorCore directly in WASM and so use
// the canonical new addresses straight off.
const PROGRAM_LOAD_ADDR: u32 = 0x0E0000;
const INPUT_LOAD_ADDR: u32 = 0x0F0000;

pub struct CompileResult {
    pub asm: String,
    pub trace: String,
    pub error: Option<CompileError>,
}

pub struct CompileError {
    pub message: String,
}

pub struct AssembleResult {
    pub listing: Vec<AssembledLine>,
    pub bytes: Vec<u8>,
    pub error: Option<String>,
}

/// Stage 1: `.f` -> `.s` via dcftn's three-phase SNOBOL4 chain.
pub fn compile(f_source: &str) -> CompileResult {
    let normalized = run_phase("normalize", NORMALIZE_SNO, f_source.as_bytes());
    let classified = run_phase("classify", CLASSIFY_SNO, normalized.output.as_bytes());
    let emitted = run_phase("emit_asm", EMIT_ASM_SNO, classified.output.as_bytes());

    let trace = format!(
        "=== normalize ({} instr, {}) ===\n{}\n\n\
         === classify ({} instr, {}) ===\n{}\n\n\
         === emit_asm ({} instr, {}) ===\n{}\n",
        normalized.instructions, normalized.stop_reason, normalized.output.trim(),
        classified.instructions, classified.stop_reason, classified.output.trim(),
        emitted.instructions,    emitted.stop_reason,    emitted.output.trim(),
    );

    // emit_asm.sno emits `; WARN: malformed input: ...` lines (older
    // builds used `*` which broke stage 2; dcftn switched to `;` so
    // they're valid COR24 comments now). Detect both prefixes and
    // treat any warn as a compile-stage failure with a friendly
    // message -- otherwise stage 2 succeeds against a partial program
    // that silently drops the unsupported statements (e.g., sum10.f
    // would print 0 instead of 55 because S = S + I is dropped).
    let warns: Vec<&str> = emitted
        .output
        .lines()
        .filter(|l| {
            let t = l.trim_start();
            t.starts_with("; WARN:") || t.starts_with("* WARN:")
        })
        .collect();

    if !warns.is_empty() {
        let detail = warns
            .iter()
            .map(|w| {
                let trimmed = w.trim_start();
                let body = trimmed
                    .trim_start_matches("; WARN: malformed input: ")
                    .trim_start_matches("* WARN: malformed input: ")
                    .trim();
                format!("  {body}")
            })
            .collect::<Vec<_>>()
            .join("\n");
        return CompileResult {
            asm: String::new(),
            error: Some(CompileError {
                message: format!(
                    "emit_asm.sno can't yet emit code for {} statement(s):\n\n{detail}\n\n\
                     dcftn shipped m3-emit-hello, which covers PROGRAM, STOP, END, and \
                     PRINT *, 'string'. Other statement kinds (INTEGER, DIMENSION, DO, \
                     GOTO, IF, integer PRINT, ASSIGN, CONTINUE) wait on subsequent \
                     milestones \u{2014} m4-print-int is in flight. See the 'Compiler \
                     trace' pane below for each phase's intermediate output.",
                    warns.len()
                ),
            }),
            trace,
        };
    }

    if looks_like_asm(&emitted.output) {
        CompileResult {
            asm: emitted.output,
            trace,
            error: None,
        }
    } else {
        CompileResult {
            asm: String::new(),
            error: Some(CompileError {
                message: "The FTI-0 compiler chain (normalize \u{2192} classify \u{2192} \
                          emit_asm) did not produce COR24 assembly. See the 'Compiler \
                          trace' pane below for each phase's intermediate output."
                    .into(),
            }),
            trace,
        }
    }
}

/// Stage 2: `.s` -> bytes via cor24-assembler.
pub fn assemble(asm: &str) -> AssembleResult {
    let mut a = Assembler::new();
    let r = a.assemble(asm);
    AssembleResult {
        listing: r.lines,
        bytes: r.bytes,
        error: if r.errors.is_empty() {
            None
        } else {
            Some(r.errors.join("\n"))
        },
    }
}

struct PhaseResult {
    output: String,
    instructions: u64,
    stop_reason: &'static str,
}

fn run_phase(_name: &str, sno_program: &str, input: &[u8]) -> PhaseResult {
    let mut emu = EmulatorCore::new();
    if let Err(e) = emu.load_lgo(SNOBOL4_LGO, None) {
        return PhaseResult {
            output: format!("(failed to load snobol4.lgo: {e})"),
            instructions: 0,
            stop_reason: "load-error",
        };
    }
    emu.load_program(PROGRAM_LOAD_ADDR, sno_program.as_bytes());
    emu.load_program(INPUT_LOAD_ADDR, input);
    emu.resume();
    let batch = emu.run_batch(SNOBOL4_PHASE_BUDGET);
    let stop_reason = match batch.reason {
        StopReason::Halted => "halted",
        StopReason::InvalidInstruction(_) => "invalid-instr",
        StopReason::Paused => "paused",
        _ => "budget-exhausted",
    };
    PhaseResult {
        output: emu.get_uart_output().to_string(),
        instructions: emu.instructions_count(),
        stop_reason,
    }
}

fn looks_like_asm(s: &str) -> bool {
    s.lines().any(|l| {
        let t = l.trim_start();
        t.starts_with(".text") || t.starts_with(".data") || t.starts_with(".globl")
    })
}
