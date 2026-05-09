# Changelog

## 2026-05-08 — Initial release: FORTRAN Hello World on COR24

First live demo of the [`web-sw-cor24-fortran`](https://github.com/sw-embed/web-sw-cor24-fortran) repo, deployed to <https://sw-embed.github.io/web-sw-cor24-fortran/>. Yew/WASM frontend that runs the upstream SNOBOL4-based FTI-0 compiler chain entirely in your browser.

### Architecture

The page is a thin shell over the upstream toolchain — there is **no** Rust-side Fortran parser. Three stages, all in your browser:

1. **Compile** — a nested `cor24-emulator` loads `snobol4.lgo` (dcsno's SNOBOL4 interpreter) and feeds it `fortran.sno` (dcftn's Fortran compiler in SNOBOL4) followed by the user's `.f` source via UART. Whatever the interpreter writes back is the candidate COR24 assembly (`.s`).
2. **Assemble** — `cor24-assembler::Assembler` over the `.s` produces machine code + listing.
3. **Run** — a fresh `cor24-emulator` executes the bytes; UART output is shown on the right.

### Path-A short-circuit

Today `fortran.sno` is dcftn's research-phase stub — for any `.f` it just prints `"FTI-0 compiler not yet implemented"`. So for the canonical `hello.f` the demo swaps in dcftn's hand-written `hello.s` (mirroring `scripts/fortran` upstream). The `.s` pane shows a "Path-A short-circuit" badge when this fires. As dcftn ships compiler phases (normalize, classify, expr, lower, emit), the demo compiles more inputs automatically — just by refreshing `assets/fortran.sno` from upstream.

### UI

- Three-column viewport-bound layout: editable source / `.s` + listing stacked / UART output.
- `Compile` / `Assemble` / `Run` buttons; demos dropdown (dcftn's four `.f` files); `[?]` modal with Usage and Reference tabs documenting the architecture.
- Collapsible "SNOBOL4 driver output" pane shows what the interpreter actually wrote back, so the research-phase stub is visible to users.
- Cohort-standard footer with build SHA / host / timestamp.

### Demos

- `hello.f` — Hello, World! (works today via Path A)
- `array1.f` — array initialization (waits on dcftn)
- `goto1.f` — GOTO loop (waits on dcftn)
- `sum10.f` — sum 1..10 (waits on dcftn)

All four are dcftn's `examples/*.f` files, vendored.

### Plumbing

- Yew + Trunk, single binary.
- Path-deps on `cor24-emulator` and `cor24-assembler`. Both must be cloned alongside this repo.
- `Trunk.toml` binds the dev server to `0.0.0.0:8414` so previews are reachable from another machine on the LAN.
- `scripts/build-pages.sh` builds `dist/` and rsyncs to `pages/`; `.github/workflows/pages.yml` deploys `pages/` on push to `main`.
- `pages/.nojekyll` committed so GitHub Pages serves the WASM/JS bundle directly.

### Bundled assets

| File | Source | Purpose |
|---|---|---|
| `assets/snobol4.lgo` | sw-cor24-snobol4 | SNOBOL4 interpreter |
| `assets/fortran.sno` | sw-cor24-fortran/snobol4/src/driver.sno | Fortran compiler in SNOBOL4 (today: stub) |
| `assets/hello.s` | sw-cor24-fortran/examples/hello.s | Path-A fixture |
| `examples/*.f` | sw-cor24-fortran/examples | FTI-0 demo programs |
