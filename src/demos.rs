//! Bundled FTI-0 demos -- dcftn's `examples/*.f` files. Today the
//! emit_asm.sno phase supports only `PRINT *, 'string'` plus
//! PROGRAM/STOP/END boilerplate (dcftn's m3-emit-hello milestone),
//! so hello.f compiles end-to-end. Inputs that use INTEGER, DO,
//! GOTO, IF, DIMENSION, or integer PRINT will run through the chain
//! but the emit_asm phase won't produce valid assembly for them yet.
//! As dcftn ships further milestones (m4 = integer PRINT, then DO /
//! GOTO / IF), refreshing `assets/emit_asm.sno` unblocks the
//! relevant demos with no UI changes.

pub const HELLO_F: &str = include_str!("../examples/hello.f");
pub const ARRAY1_F: &str = include_str!("../examples/array1.f");
pub const GOTO1_F: &str = include_str!("../examples/goto1.f");
pub const SUM10_F: &str = include_str!("../examples/sum10.f");

pub struct Demo {
    pub id: &'static str,
    pub label: &'static str,
    pub source: &'static str,
}

/// Demos that compile + assemble + run end-to-end against the current
/// emit_asm.sno (m3-emit-hello: PROGRAM, STOP, END, PRINT *, 'string').
pub const WORKING: &[Demo] = &[
    Demo { id: "hello.f", label: "Hello, World!", source: HELLO_F },
];

/// Demos that run through the chain but emit_asm.sno doesn't yet know
/// how to emit code for their statement kinds. Picked from upstream
/// `sw-cor24-fortran/examples/` so they unlock as dcftn ships further
/// milestones (m4 = integer PRINT, then DO / GOTO / IF / INTEGER /
/// DIMENSION).
pub const PENDING: &[Demo] = &[
    Demo { id: "array1.f", label: "Array init  \u{2014} needs DIMENSION + integer PRINT",  source: ARRAY1_F },
    Demo { id: "goto1.f",  label: "GOTO loop   \u{2014} needs INTEGER + GOTO + IF",         source: GOTO1_F },
    Demo { id: "sum10.f",  label: "Sum 1..10   \u{2014} needs INTEGER + DO + integer PRINT", source: SUM10_F },
];

pub fn lookup(id: &str) -> Option<&'static str> {
    WORKING
        .iter()
        .chain(PENDING.iter())
        .find(|d| d.id == id)
        .map(|d| d.source)
}

pub const DEFAULT_SOURCE: &str = HELLO_F;
