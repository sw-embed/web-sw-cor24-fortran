//! Bundled FTI-0 demos. These are dcftn's `examples/*.f` files,
//! exercising the upstream Fortran compiler. Today only `hello.f`
//! compiles end-to-end (via the Path-A short-circuit in
//! `compiler.rs`); the others print dcftn's research-phase stub
//! message. As dcftn ships fortran.sno phases, the others start
//! compiling automatically.

pub const HELLO_F: &str = include_str!("../examples/hello.f");
pub const ARRAY1_F: &str = include_str!("../examples/array1.f");
pub const GOTO1_F: &str = include_str!("../examples/goto1.f");
pub const SUM10_F: &str = include_str!("../examples/sum10.f");

pub struct Demo {
    pub id: &'static str,
    pub label: &'static str,
    pub source: &'static str,
}

pub const DEMOS: &[Demo] = &[
    Demo { id: "hello.f",  label: "Hello, World!",                source: HELLO_F },
    Demo { id: "array1.f", label: "Array initialization (waits)", source: ARRAY1_F },
    Demo { id: "goto1.f",  label: "GOTO loop (waits)",            source: GOTO1_F },
    Demo { id: "sum10.f",  label: "Sum 1..10 (waits)",            source: SUM10_F },
];

pub fn lookup(id: &str) -> Option<&'static str> {
    DEMOS.iter().find(|d| d.id == id).map(|d| d.source)
}

pub const DEFAULT_SOURCE: &str = HELLO_F;
