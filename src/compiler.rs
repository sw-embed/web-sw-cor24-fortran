//! Three-stage in-browser pipeline:
//!
//!   stage 1: `.f` -> `.s`
//!     Runs the SNOBOL4 interpreter (`snobol4.lgo`, dcsno's artifact)
//!     in a nested `cor24_emulator::EmulatorCore`, with dcftn's
//!     Fortran-compiler SNOBOL4 program (`fortran.sno`) followed by
//!     the user's `.f` source as UART input. Whatever the interpreter
//!     writes to UART is the candidate `.s`. **No Rust-side Fortran
//!     parsing.** This is just a runner.
//!
//!     Today `fortran.sno` is dcftn's research-phase stub
//!     (`driver.sno`: `OUTPUT = 'FTI-0 compiler not yet implemented'`),
//!     so for any `.f` the SNOBOL4 driver emits the stub line. We
//!     surface that in the UI verbatim so the user sees what
//!     dcftn ships today. As dcftn matures `fortran.sno`, the demo
//!     compiles more inputs automatically with zero UI changes.
//!
//!     Path A (short-circuit): for the canonical `hello.f` content
//!     match, swap in dcftn's pre-baked `hello.s` so the demo has at
//!     least one program that flows through every stage today. This
//!     mirrors what `scripts/fortran` does in `sw-cor24-fortran`.
//!
//!   stage 2: `.s` -> bytes via `cor24_assembler::Assembler`.
//!   stage 3: bytes -> UART output via `cor24_emulator::EmulatorCore`.

use cor24_assembler::{AssembledLine, Assembler};
use cor24_emulator::{EmulatorCore, StopReason};

pub const SNOBOL4_LGO: &str = include_str!("../assets/snobol4.lgo");
pub const FORTRAN_SNO: &str = include_str!("../assets/fortran.sno");
pub const HELLO_S_PATH_A: &str = include_str!("../assets/hello.s");
pub const HELLO_F_CANONICAL: &str = include_str!("../examples/hello.f");

const SNOBOL4_BUDGET: u64 = 50_000_000;

pub struct CompileResult {
    pub asm: String,
    pub driver_log: String,
    pub via_path_a: bool,
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

pub fn compile(f_source: &str) -> CompileResult {
    let driver_log = run_snobol4_compiler(f_source);

    if looks_like_asm(&driver_log) {
        return CompileResult {
            asm: driver_log.clone(),
            driver_log,
            via_path_a: false,
            error: None,
        };
    }

    if matches_canonical_hello(f_source) {
        return CompileResult {
            asm: HELLO_S_PATH_A.to_string(),
            driver_log,
            via_path_a: true,
            error: None,
        };
    }

    CompileResult {
        asm: String::new(),
        error: Some(CompileError {
            message: format!(
                "fortran.sno (running on snobol4.lgo) did not produce assembly. \
                 The driver said:\n\n{}\n\n\
                 dcftn's FTI-0 compiler is research-phase today (driver.sno is a stub \
                 that prints 'FTI-0 compiler not yet implemented'). Only hello.f \
                 compiles end-to-end via the Path-A short-circuit. As dcftn matures \
                 fortran.sno, this demo will compile more inputs automatically \
                 with no changes to this page \u{2014} just a refreshed assets/fortran.sno.",
                driver_log.trim()
            ),
        }),
        driver_log,
        via_path_a: false,
    }
}

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

fn run_snobol4_compiler(f_source: &str) -> String {
    let mut emu = EmulatorCore::new();
    if let Err(e) = emu.load_lgo(SNOBOL4_LGO, None) {
        return format!("(failed to load snobol4.lgo: {e})");
    }

    for byte in FORTRAN_SNO.bytes() {
        emu.send_uart_byte(byte);
    }
    emu.send_uart_byte(0x04);

    for byte in f_source.bytes() {
        emu.send_uart_byte(byte);
    }
    emu.send_uart_byte(0x04);

    emu.resume();
    let batch = emu.run_batch(SNOBOL4_BUDGET);

    let suffix = match batch.reason {
        StopReason::Halted => "",
        StopReason::InvalidInstruction(_) => "\n(snobol4 emulator halted on invalid instruction)",
        StopReason::Paused => "\n(snobol4 emulator paused before completion)",
        _ => "\n(snobol4 emulator did not run to completion within instruction budget)",
    };
    format!("{}{suffix}", emu.get_uart_output())
}

fn looks_like_asm(s: &str) -> bool {
    s.lines().any(|l| {
        let t = l.trim_start();
        t.starts_with(".text") || t.starts_with(".data") || t.starts_with(".globl")
    })
}

fn matches_canonical_hello(f: &str) -> bool {
    let norm = |s: &str| {
        s.replace('\r', "")
            .lines()
            .map(|l| l.trim_end())
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string()
    };
    norm(f) == norm(HELLO_F_CANONICAL)
}
