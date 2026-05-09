//! Three-stage compile pipeline:
//!
//!   stage 1: FTI-0 source (`.f`) -> COR24 assembly (`.s`)
//!     A small Rust compiler that handles a deliberately tiny FTI-0
//!     subset (see help dialog Reference tab):
//!       - column-1 'C' / 'c' / '*' line comments
//!       - PROGRAM <name>
//!       - PRINT *, 'string-literal' (Fortran-style '' escape)
//!       - STOP, END
//!       - optional 1..5-column statement labels
//!     Anything else returns a line-numbered error.
//!
//!   stage 2: `.s` -> machine code via cor24-assembler.
//!   stage 3: machine code -> UART output via cor24-emulator.

use cor24_assembler::{AssembledLine, Assembler};

pub struct CompileResult {
    pub asm: String,
    pub error: Option<CompileError>,
}

pub struct CompileError {
    pub message: String,
    pub line: Option<usize>,
}

pub struct AssembleResult {
    pub listing: Vec<AssembledLine>,
    pub bytes: Vec<u8>,
    pub error: Option<String>,
}

/// Stage 1: parse FTI-0, emit COR24 assembly.
pub fn compile(f_source: &str) -> CompileResult {
    let mut prints: Vec<String> = Vec::new();

    for (idx, raw_line) in f_source.split('\n').enumerate() {
        let line_no = idx + 1;
        let bytes = raw_line.as_bytes();
        if bytes.is_empty() || raw_line.trim().is_empty() {
            continue;
        }
        if matches!(bytes[0], b'C' | b'c' | b'*') {
            continue;
        }

        let stmt = if bytes.len() >= 6 {
            raw_line[6..].trim().to_string()
        } else {
            raw_line.trim().to_string()
        };

        let stmt_upper = stmt.to_ascii_uppercase();
        if stmt_upper.is_empty() {
            continue;
        }
        if stmt_upper.starts_with("PROGRAM") || stmt_upper == "STOP" || stmt_upper == "END" {
            continue;
        }
        if stmt_upper.starts_with("PRINT") {
            match parse_print_statement(&stmt) {
                Ok(args) => prints.push(format_print_args(&args)),
                Err(e) => {
                    return CompileResult {
                        asm: String::new(),
                        error: Some(CompileError {
                            message: format!("PRINT statement: {e}"),
                            line: Some(line_no),
                        }),
                    };
                }
            }
            continue;
        }

        return CompileResult {
            asm: String::new(),
            error: Some(CompileError {
                message: format!(
                    "Unrecognised statement: {stmt:?}. This demo's compiler accepts only \
                     PROGRAM, PRINT *, 'string', STOP, and END today (see Help \u{2192} \
                     Reference)."
                ),
                line: Some(line_no),
            }),
        };
    }

    CompileResult {
        asm: emit_asm(&prints),
        error: None,
    }
}

enum PrintArg {
    Str(String),
    Int(i64),
}

fn parse_print_statement(stmt: &str) -> Result<Vec<PrintArg>, String> {
    let after_print = stmt
        .get(5..)
        .ok_or("expected 'PRINT *, ...'")?
        .trim_start();
    let after_star = after_print
        .strip_prefix('*')
        .ok_or("expected '*' after PRINT")?
        .trim_start();
    let mut rest = after_star
        .strip_prefix(',')
        .ok_or("expected ',' after PRINT *")?
        .trim_start();

    let mut args = Vec::new();
    loop {
        rest = rest.trim_start();
        if rest.is_empty() {
            return Err("expected argument after ','".into());
        }
        let (arg, after) = parse_print_arg(rest)?;
        args.push(arg);
        rest = after.trim_start();
        if rest.is_empty() {
            return Ok(args);
        }
        rest = rest
            .strip_prefix(',')
            .ok_or_else(|| format!("expected ',' between PRINT arguments at {rest:?}"))?;
    }
}

fn parse_print_arg(s: &str) -> Result<(PrintArg, &str), String> {
    if s.starts_with('\'') {
        let (lit, after) = parse_string_literal(s)?;
        Ok((PrintArg::Str(lit), after))
    } else {
        let (n, after) = parse_int_expr(s)?;
        Ok((PrintArg::Int(n), after))
    }
}

fn parse_string_literal(s: &str) -> Result<(String, &str), String> {
    let bytes = s.as_bytes();
    let mut i = 1; // skip opening '
    let mut out = String::new();
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'\'' {
            if i + 1 < bytes.len() && bytes[i + 1] == b'\'' {
                out.push('\'');
                i += 2;
                continue;
            }
            return Ok((out, &s[i + 1..]));
        }
        out.push(b as char);
        i += 1;
    }
    Err("unterminated string literal".into())
}

fn format_print_args(args: &[PrintArg]) -> String {
    let mut s = String::new();
    let mut first = true;
    for a in args {
        if !first {
            s.push(' ');
        }
        first = false;
        match a {
            PrintArg::Str(t) => s.push_str(t),
            PrintArg::Int(n) => s.push_str(&n.to_string()),
        }
    }
    s
}

// Recursive-descent integer expression evaluator (compile-time).
//   expr   = term  (('+'|'-') term)*
//   term   = factor (('*'|'/') factor)*
//   factor = '-' factor | '(' expr ')' | integer-literal
fn parse_int_expr(s: &str) -> Result<(i64, &str), String> {
    let (mut acc, mut rest) = parse_term(s)?;
    loop {
        let r = rest.trim_start();
        if let Some(after) = r.strip_prefix('+') {
            let (n, after) = parse_term(after)?;
            acc = acc.wrapping_add(n);
            rest = after;
        } else if let Some(after) = r.strip_prefix('-') {
            let (n, after) = parse_term(after)?;
            acc = acc.wrapping_sub(n);
            rest = after;
        } else {
            return Ok((acc, rest));
        }
    }
}

fn parse_term(s: &str) -> Result<(i64, &str), String> {
    let (mut acc, mut rest) = parse_factor(s)?;
    loop {
        let r = rest.trim_start();
        if let Some(after) = r.strip_prefix('*') {
            let (n, after) = parse_factor(after)?;
            acc = acc.wrapping_mul(n);
            rest = after;
        } else if let Some(after) = r.strip_prefix('/') {
            let (n, after) = parse_factor(after)?;
            if n == 0 {
                return Err("division by zero".into());
            }
            acc /= n;
            rest = after;
        } else {
            return Ok((acc, rest));
        }
    }
}

fn parse_factor(s: &str) -> Result<(i64, &str), String> {
    let s = s.trim_start();
    if let Some(after) = s.strip_prefix('(') {
        let (n, after) = parse_int_expr(after)?;
        let after = after
            .trim_start()
            .strip_prefix(')')
            .ok_or("expected ')'")?;
        return Ok((n, after));
    }
    if let Some(after) = s.strip_prefix('-') {
        let (n, after) = parse_factor(after)?;
        return Ok((-n, after));
    }
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if i == 0 {
        return Err(format!("expected integer or '(' at {s:?}"));
    }
    let n: i64 = s[..i]
        .parse()
        .map_err(|e: std::num::ParseIntError| e.to_string())?;
    Ok((n, &s[i..]))
}

fn emit_asm(prints: &[String]) -> String {
    let mut s = String::new();
    s.push_str(HARNESS);
    s.push_str("        .globl  _main\n");
    s.push_str("_main:\n");
    s.push_str("        push    fp\n");
    s.push_str("        push    r2\n");
    s.push_str("        push    r1\n");
    s.push_str("        mov     fp,sp\n");

    for i in 0..prints.len() {
        s.push_str(&format!("        la      r0,_S{i}\n"));
        s.push_str("        push    r0\n");
        s.push_str("        la      r0,_puts\n");
        s.push_str("        jal     r1,(r0)\n");
        s.push_str("        add     sp,3\n");
    }

    s.push_str("        mov     sp,fp\n");
    s.push_str("        pop     r1\n");
    s.push_str("        pop     r2\n");
    s.push_str("        pop     fp\n");
    s.push_str("        jmp     (r1)\n\n");

    s.push_str("        .data\n");
    for (i, lit) in prints.iter().enumerate() {
        s.push_str(&format!("_S{i}:\n        .byte   "));
        let mut first = true;
        for ch in lit.bytes() {
            if !first {
                s.push(',');
            }
            s.push_str(&ch.to_string());
            first = false;
        }
        if !first {
            s.push(',');
        }
        s.push_str("10,0\n");
    }
    s
}

const HARNESS: &str = r#"        .text

        .globl  _start
_start:
        la      r0,_main
        jal     r1,(r0)
_halt:
        bra     _halt

        .globl  _putc
_putc:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
L_pc_wait:
        la      r0,16711937
        lbu     r0,0(r0)
        push    r0
        la      r0,128
        mov     r1,r0
        pop     r0
        and     r0,r1
        ceq     r0,z
        brt     L_pc_send
        bra     L_pc_wait
L_pc_send:
        lw      r0,9(fp)
        push    r0
        la      r0,16711936
        mov     r1,r0
        pop     r0
        sb      r0,0(r1)
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _puts
_puts:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
L_ps_loop:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brt     L_ps_done
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        la      r0,_putc
        jal     r1,(r0)
        add     sp,3
        lw      r0,9(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,9(fp)
        bra     L_ps_loop
L_ps_done:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

"#;

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
