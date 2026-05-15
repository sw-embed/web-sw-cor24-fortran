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
| `assets/{normalize,classify,emit_asm}.sno` | `work/lib/cor24/fortran/snobol4/src/` | `cp` from there |
| `examples/*.f` | `sw-cor24-fortran/examples` | `cp` from there (or `git show origin/dev:examples/...`) |

As of dcftn m13-inline-runtime (2026-05-14), `emit_asm.sno` emits the
full .s including the runtime support routines (`_start / _halt /
_putc / _puts / _putint / _aindex`). No more `; __RUNTIME_PRELUDE__` /
`; __RUNTIME_PUTINT__` marker splicing &mdash; that workaround was needed
when emit_asm.sno hit dcsno's source-byte cap. dcsno's
`pr/cap-and-pattern-fixes` bumped internal buffers; emit_asm now
fits even at 18 KB+.

Load-address convention (post-2026-05-14 dcsno):
- compiler source @ `0xE0000` (was `0x080000`)
- input data @ `0xF0000` (was `0x090000`)

Hardcoded in `src/compiler.rs` as `PROGRAM_LOAD_ADDR` / `INPUT_LOAD_ADDR`.
If dcsno migrates again, update those two constants.

After refreshing assets, run `./scripts/build-pages.sh` to rebake
`pages/` and commit.

## Asset refresh routine (READ THIS BEFORE SHIPPING)

The canonical source for `.sno` / `.s` runtime files is
`work/lib/cor24/fortran/snobol4/` &mdash; mike maintains it. Hit-once-and-done
copies have bitten us: mike sometimes re-installs the same files
between `dg-new-feature` and `git commit`, leaving stale assets in
this repo. Always do this in order:

1. `cp` from `work/lib/cor24/fortran/snobol4/{src,runtime}/*` into
   `assets/`.
2. **`diff` `assets/*` against the install dir.** If non-empty, mike
   has done a follow-up install &mdash; re-copy and re-verify.
3. End-to-end test each demo through the upstream wrapper. Easy
   one-liner under `/tmp`:

   ```bash
   cd /tmp && cp <repo>/assets/{normalize,classify,emit_asm}.sno .
   for f in <repo>/examples/*.f; do
       snobol4 --load-binary normalize.sno@0x080000 --load-binary "$f@0x090000" \
               --entry 0 --quiet --speed 0 -n 100000000 2>/dev/null > /tmp/n.txt
       snobol4 --load-binary classify.sno@0x080000 --load-binary /tmp/n.txt@0x090000 \
               --entry 0 --quiet --speed 0 -n 100000000 2>/dev/null > /tmp/c.txt
       snobol4 --load-binary emit_asm.sno@0x080000 --load-binary /tmp/c.txt@0x090000 \
               --entry 0 --quiet --speed 0 -n 100000000 2>/dev/null > /tmp/asm.txt
       awk -v p=<repo>/assets/prelude.s -v u=<repo>/assets/putint.s \
           '/__RUNTIME_PRELUDE__/ {while ((getline l < p) > 0) print l; close(p); next}
            /__RUNTIME_PUTINT__/  {while ((getline l < u) > 0) print l; close(u);  next}
            {print}' /tmp/asm.txt > /tmp/full.s
       D=$(mktemp -d) && cp /tmp/full.s "$D/" && (cd "$D" && cor24-asm full.s -o full.lgo \
           && cor24-emu --lgo full.lgo --quiet --speed 0 -n 10000000) && rm -rf "$D"
   done
   ```

4. Only after all demos produce expected output: `./scripts/build-pages.sh`,
   commit, `dg-mark-pr`.

`snobol4.lgo` is canonical at `work/lib/cor24/snobol4.lgo` (one level
up from the fortran dir). Same diff-then-copy discipline applies.

### Why this matters

dcftn ships emit_asm.sno changes in waves &mdash; sometimes the milestone PR
merges, then a minor follow-up fix lands minutes later and mike
re-installs without a new merge commit. The git log of `sw-cor24-fortran`
won't reflect it; only the timestamps under `work/lib/cor24/fortran/`
do. Skipping the diff step **silently** ships a stale compiler.

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
