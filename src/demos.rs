//! Bundled FTI-0 demos -- dcftn's `examples/*.f` files. Listed in
//! milestone order so the simplest comes first. As of m9 the full
//! original demo set compiles end-to-end. As dcftn adds more
//! milestones (subroutines, more types, I/O, ...), append new demos
//! here and refresh the upstream `.sno` assets.

pub const HELLO_F: &str = include_str!("../examples/hello.f");
pub const PRINT_INT_F: &str = include_str!("../examples/print-int.f");
pub const PRINT_VAR_F: &str = include_str!("../examples/print-var.f");
pub const ADD_F: &str = include_str!("../examples/add.f");
pub const GOTO1_F: &str = include_str!("../examples/goto1.f");
pub const SUM10_F: &str = include_str!("../examples/sum10.f");
pub const ARRAY1_F: &str = include_str!("../examples/array1.f");
pub const FACTORIAL_F: &str = include_str!("../examples/factorial.f");
pub const FIBONACCI_F: &str = include_str!("../examples/fibonacci.f");
pub const FIZZBUZZ_F: &str = include_str!("../examples/fizzbuzz.f");

pub struct Demo {
    pub id: &'static str,
    pub label: &'static str,
    pub source: &'static str,
}

pub const WORKING: &[Demo] = &[
    Demo { id: "hello.f",     label: "Hello, World!  (m3-emit-hello)",                  source: HELLO_F },
    Demo { id: "print-int.f", label: "Print int literal  (m4-print-int)",               source: PRINT_INT_F },
    Demo { id: "print-var.f", label: "Print int variable  (m5-print-var)",              source: PRINT_VAR_F },
    Demo { id: "add.f",       label: "Binary addition  (m6-assign-expr)",               source: ADD_F },
    Demo { id: "goto1.f",     label: "GOTO loop, count to 5  (m7-goto)",                source: GOTO1_F },
    Demo { id: "sum10.f",     label: "Sum 1..10 with DO  (m8-do-loop)",                 source: SUM10_F },
    Demo { id: "array1.f",    label: "Array DIMENSION + indexing  (m9-array)",          source: ARRAY1_F },
    Demo { id: "factorial.f", label: "5! via DO loop  (m11-demos)",                     source: FACTORIAL_F },
    Demo { id: "fibonacci.f", label: "Iterative fib(11) = 89  (m11-demos)",             source: FIBONACCI_F },
    Demo { id: "fizzbuzz.f",  label: "FizzBuzz 1..15 (counters, no div/mod)  (m11-demos)", source: FIZZBUZZ_F },
];

/// No demos currently waiting on dcftn -- m9 closed out the original
/// four. Add new entries here as dcftn ships milestones that the
/// existing `.sno` chain doesn't yet handle.
pub const PENDING: &[Demo] = &[];

pub fn lookup(id: &str) -> Option<&'static str> {
    WORKING
        .iter()
        .chain(PENDING.iter())
        .find(|d| d.id == id)
        .map(|d| d.source)
}

pub const DEFAULT_SOURCE: &str = HELLO_F;
