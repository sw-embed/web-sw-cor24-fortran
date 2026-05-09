# Exploration archive

Files in this directory are **not part of the v1 demo**. They are
preserved here as a reference for a possible future "richer Fortran
demo" saga.

The v1 saga (`dwftn-hello-world-demo`) is intentionally narrow per
its brief: a Yew/WASM page that embeds `examples/hello.lgo` (built by
upstream `sw-cor24-fortran`) and runs it inside the COR24 emulator.
That page lives in `src/main.rs`.

## Why these files exist

While building the v1 demo I went off-brief and built a Rust-based
mini-compiler for a tiny FTI-0 subset (`PRINT *, 'string'` plus
compile-time integer arithmetic) along with an editor, syntax
highlighter, help dialog, and listing/UART panels. The brief was
explicit:

> "No live compilation in the browser. The .lgo is pre-built."
> "No multiple Fortran demos. Just hello world."
> "No editor/REPL UI — read-only display is fine for v1."

Shipping a Rust Fortran compiler would have introduced a *third*
Fortran compiler in the project (alongside dcftn's hand-written
`hello.s` short-circuit and the future SNOBOL4-based one), which is
the wrong pattern. The demo's job is to **consume** dcftn's compiler
output, not reimplement the compiler in Rust.

The work is archived rather than deleted because some of it may be
useful when a future saga upgrades this demo to a richer experience —
but only after dcftn's SNOBOL4-based compiler is mature enough that
the demo can consume *that* (running `snobol4.lgo` + the Fortran
compiler `.sno` in a nested COR24 emulator).

## Contents

- `compiler.rs` — Rust mini-compiler covering column-1 comments,
  `PROGRAM`/`STOP`/`END`, and `PRINT *, args` where each arg is a
  string literal or a compile-time-evaluated integer expression.
- `editor.rs` — transparent-textarea-over-`<pre>` editor component
  with line gutter and error-line highlighting.
- `highlight.rs` — Fortran fixed-form syntax highlighter (column-1
  comments, keywords, numbers, strings).
- `demos.rs` — five bundled `.f` programs (hello, greeting, quote,
  math, arithmetic).
- `help.rs` — modal dialog with Usage / Reference tabs.
- `panels/listing.rs` — assembler listing renderer.
- `panels/uart.rs` — UART output panel.

## If you resurrect any of these

- The brief for the future saga should explicitly say that the demo
  consumes dcftn's SNOBOL4-based compiler (loaded into a nested
  `cor24-emulator` instance), not a parallel Rust implementation.
- Stage 1 = run `snobol4.lgo` with the Fortran-compiler `.sno`
  source + user `.f` via UART, capture the emitted assembly.
- Stage 2 = `cor24-assembler::Assembler::assemble(.s)`.
- Stage 3 = `cor24_emulator::EmulatorCore` running the bytes.
- Drop the Rust mini-compiler in `compiler.rs`; replace with the
  nested-emulator stage-1 invocation.
