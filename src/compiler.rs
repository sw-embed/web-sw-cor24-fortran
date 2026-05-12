//! Three-stage in-browser pipeline. Stage 1 chains dcftn's three
//! SNOBOL4-implemented compiler phases (no Rust-side Fortran parser):
//!
//!   user .f
//!     |   nested cor24-emu loads snobol4.lgo,
//!     |   loads normalize.sno @ 0x080000 and .f @ 0x090000,
//!     |   runs -> emits normalized statement records via UART
//!     v
//!   normalized records
//!     |   nested cor24-emu loads snobol4.lgo,
//!     |   loads classify.sno @ 0x080000 and records @ 0x090000,
//!     |   runs -> emits records with kind= field added
//!     v
//!   classified records
//!     |   nested cor24-emu loads snobol4.lgo,
//!     |   loads emit_asm.sno @ 0x080000 and records @ 0x090000,
//!     |   runs -> emits COR24 assembly
//!     v
//!   .s assembly
//!     |   cor24-assembler -> bytes + listing  (stage 2)
//!     |   cor24-emulator -> UART output       (stage 3)
//!     v
//!   program output
//!
//! Each compiler phase is a separate `EmulatorCore` invocation that
//! exactly mirrors what `scripts/fortran` does in `sw-cor24-fortran`
//! (which runs three `snobol4 --load-binary <phase>.sno@0x080000
//! --load-binary <data>@0x090000` commands).
//!
//! Today the chain handles only the FTI-0 subset that emit_asm.sno
//! supports: PROGRAM / STOP / END boilerplate, and `PRINT *, 'string'`.
//! As dcftn ships further phases (m4 adds integer PRINT, future
//! milestones add DO / GOTO / IF / INTEGER / DIMENSION), refreshing
//! `assets/*.sno` is the only change needed here.

use cor24_assembler::{AssembledLine, Assembler};
use cor24_emulator::{EmulatorCore, StopReason};

pub const SNOBOL4_LGO: &str = include_str!("../assets/snobol4.lgo");
pub const NORMALIZE_SNO: &str = include_str!("../assets/normalize.sno");
pub const CLASSIFY_SNO: &str = include_str!("../assets/classify.sno");
pub const EMIT_ASM_SNO: &str = include_str!("../assets/emit_asm.sno");

const SNOBOL4_PHASE_BUDGET: u64 = 200_000_000;
const INPUT_LOAD_ADDR: u32 = 0x090000;
const PROGRAM_LOAD_ADDR: u32 = 0x080000;

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
                message: format!(
                    "The FTI-0 compiler chain (normalize \u{2192} classify \u{2192} emit_asm) \
                     did not produce COR24 assembly. The emit_asm phase supports only the \
                     subset of FTI-0 that dcftn has shipped so far (PROGRAM/STOP/END plus \
                     PRINT *, 'string'). Inputs using INTEGER declarations, DO loops, \
                     GOTO, IF, DIMENSION, or integer PRINT will not yet compile.\n\n\
                     See the 'Compiler trace' pane below for what each phase emitted."
                ),
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
