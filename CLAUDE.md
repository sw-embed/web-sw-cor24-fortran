# CLAUDE.md

Context for AI agents working in this repo.

## Purpose

This is the web frontend for the COR24 FORTRAN hello-world live demo.
It embeds a pre-built `.lgo` (produced upstream by
`sw-cor24-fortran`) and runs it in a WASM-compiled COR24 emulator
inside the browser. It does **not** compile Fortran itself.

## Architecture

- `src/main.rs` &mdash; single-file Yew app. On mount, loads
  `examples/hello.lgo` (via `include_bytes!`) into a
  `cor24_emulator::EmulatorCore`, runs it batched per
  `gloo_timers::Interval`, and renders the UART output. If
  `hello.lgo` is empty (placeholder mode), shows a "pending" notice.
- `examples/hello.f` &mdash; the FORTRAN source, displayed read-only
  on the left.
- `examples/hello.lgo` &mdash; the pre-built loadable object. Begins
  life as a 0-byte placeholder; dcftn's `sw-cor24-fortran` saga
  produces the real binary.
- `scripts/build-pages.sh` &mdash; `trunk build --release` &rarr;
  `pages/`, then commit. CI deploys `pages/` to GitHub Pages.

## Path-deps

`Cargo.toml` references `cor24-emulator` and `cor24-isa` as path-deps
in the parent directory:

```
../sw-cor24-emulator
../sw-cor24-isa
```

Both must be cloned alongside this repo before `cargo check` /
`trunk build` will work.

## Workflow

This repo follows the standard saga workflow:

1. `dg-new-feature <slug>` from `dev`.
2. Implement, commit.
3. `dg-mark-pr` to ready for relay.
4. mike relays to `main`.
5. CI deploys.

## What does NOT belong here

- A Fortran compiler. That lives upstream at
  [`sw-cor24-fortran`](https://github.com/sw-embed/sw-cor24-fortran).
- Multiple demos. v1 is hello-world only; future demos will be added
  in subsequent sagas as the upstream compiler matures.
- An editor / REPL. The source is read-only display.
