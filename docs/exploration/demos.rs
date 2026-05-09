//! Bundled FTI-0 demos. All four exercise the Rust-based stage-1 subset
//! (column-1 comments, PROGRAM/STOP/END, PRINT *, 'string') so editing
//! and recompiling actually works end-to-end.

pub const HELLO_F: &str = include_str!("../examples/hello.f");
pub const GREETING_F: &str = include_str!("../examples/greeting.f");
pub const QUOTE_F: &str = include_str!("../examples/quote.f");
pub const MATH_F: &str = include_str!("../examples/math.f");
pub const ARITHMETIC_F: &str = include_str!("../examples/arithmetic.f");

pub struct Demo {
    pub id: &'static str,
    pub label: &'static str,
    pub source: &'static str,
}

pub const DEMOS: &[Demo] = &[
    Demo { id: "hello.f",      label: "Hello, World!",            source: HELLO_F },
    Demo { id: "greeting.f",   label: "Multi-line greeting",      source: GREETING_F },
    Demo { id: "quote.f",      label: "Embedded quote (escape)",  source: QUOTE_F },
    Demo { id: "math.f",       label: "Arithmetic in PRINT",      source: MATH_F },
    Demo { id: "arithmetic.f", label: "Operator showcase",        source: ARITHMETIC_F },
];

pub fn lookup(id: &str) -> Option<&'static str> {
    DEMOS.iter().find(|d| d.id == id).map(|d| d.source)
}

pub const DEFAULT_SOURCE: &str = HELLO_F;
