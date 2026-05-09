# CLAUDE.md

Context for AI agents working in this repo.

## Purpose

This is the web frontend for the COR24 FORTRAN live demo. It runs the
upstream SNOBOL4-based FTI-0 compiler chain entirely in your browser:
`cor24-emulator` loads `snobol4.lgo`, which interprets `fortran.sno`
(dcftn's compiler), which compiles user `.f` source to COR24 `.s`. A
second `cor24-emulator` then runs the assembled bytes.

**There is no Rust-side Fortran parser.** Implementing one would
introduce a third Fortran compiler in the project (alongside dcftn's
hand-written `hello.s` and the future SNOBOL4-based one) — wrong
pattern. The web-* repos consume the upstream compiler's output;
they do not reimplement it.

## Architecture

```
user .f source  --[ Compile  ]-->  .s        (snobol4.lgo runs fortran.sno on user .f)
   .s           --[ Assemble ]-->  bytes     (cor24-assembler in this WASM bundle)
   bytes        --[ Run      ]-->  UART out  (cor24-emulator in this WASM bundle)
```

- `src/main.rs` — Yew app, three buttons (Compile / Assemble / Run),
  three-column viewport-bound layout, demos dropdown, `[?]` help modal.
  No auto-run on mount.
- `src/compiler.rs` — stage 1 (`run_snobol4_compiler`: nested
  `EmulatorCore` loads `snobol4.lgo`, feeds `fortran.sno` + user `.f`
  via UART, captures output) and stage 2 (`assemble`:
  `cor24-assembler::Assembler::assemble`). Stage 3 lives in `main.rs`'s
  run loop.
- `src/editor.rs`, `src/highlight.rs` — editable source pane with
  Fortran fixed-form highlighting (column-1 comments, keywords).
- `src/panels/{listing,uart}.rs` — listing renderer + UART output panel.
- `src/help.rs` — Usage / Reference modal.
- `src/demos.rs` — dcftn's four `.f` demos vendored.

## Path-A short-circuit

Today `assets/fortran.sno` is dcftn's research-phase stub
(`driver.sno`: prints `"FTI-0 compiler not yet implemented"`). The
demo runs it for every `.f` to surface the actual upstream state in
the SNOBOL4-driver-output pane. For the canonical `hello.f` content
match, the demo additionally swaps in dcftn's hand-written
`assets/hello.s` so the page has at least one program that flows
through every stage today. This mirrors `scripts/fortran` in the
upstream `sw-cor24-fortran` repo. As dcftn ships compiler phases,
the short-circuit becomes irrelevant and goes away.

## Bundled assets

| File | Source upstream | Refresh procedure |
|---|---|---|
| `assets/snobol4.lgo` | `work/lib/cor24/snobol4.lgo` (from dcsno) | `cp` from there |
| `assets/fortran.sno` | `sw-cor24-fortran/snobol4/src/driver.sno` | `cp` from there (or whatever bundled form dcftn settles on) |
| `assets/hello.s` | `sw-cor24-fortran/examples/hello.s` | `cp` from there |
| `examples/*.f` | `sw-cor24-fortran/examples` | `cp` from there |

After refreshing assets, run `./scripts/build-pages.sh` to rebake
`pages/` and commit.

## Path-deps

`Cargo.toml` references two sibling crates:

```
../sw-cor24-emulator
../sw-cor24-x-assembler
```

Both must be cloned alongside this repo before `cargo check` /
`trunk build` will work.

## Workflow

Standard saga lifecycle:

1. `dg-new-feature <slug>` from `dev`.
2. Implement on `feat/<slug>`.
3. `dg-mark-pr` renames to `pr/<slug>` when ready.
4. mike relays to `main`.
5. CI deploys `pages/` to GitHub Pages.

## What does NOT belong here

- A Rust Fortran compiler. dcftn ships `fortran.sno` (in SNOBOL4);
  this page consumes that.
- Hand-written demo assembly beyond the existing Path-A `hello.s`
  fallback. As dcftn matures, even that fallback should retire.
- New demo `.f` files invented here. Upstream's `examples/` is the
  source of truth.
