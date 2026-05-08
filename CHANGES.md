# Changelog

## 2026-05-08 — Initial release: FORTRAN Hello World on COR24

First live demo of the [`web-sw-cor24-fortran`](https://github.com/sw-embed/web-sw-cor24-fortran) repo, deployed to <https://sw-embed.github.io/web-sw-cor24-fortran/>. Yew/WASM frontend with a three-stage in-browser pipeline.

### Pipeline

- **Compile** — Rust-based FTI-0 mini-compiler (`src/compiler.rs`) turns user `.f` source into COR24 assembly.
- **Assemble** — `cor24-assembler::Assembler` produces machine code + listing.
- **Run** — `cor24-emulator::EmulatorCore` executes the bytes; UART output renders on the right.

### Supported FTI-0 subset

- Column-1 `C` / `c` / `*` line comments.
- `PROGRAM <name>`, `STOP`, `END` (consumed; emit no code).
- Optional 1..5-column statement labels (consumed).
- `PRINT *, arg1, arg2, ...` where each arg is either:
  - a single-quoted string literal (with Fortran `''` escape for embedded quote), or
  - an integer expression: literals, `+ - * /`, parens, unary minus &mdash; evaluated at compile time.

Five bundled demos exercise the subset: `hello.f`, `greeting.f`, `quote.f`, `math.f`, `arithmetic.f`.

### UI

- Three-column viewport-bound layout: editable source / `.s` + listing stacked / UART output.
- `Compile` / `Assemble` / `Run` buttons; demos dropdown; `[?]` modal with Usage and Reference tabs.
- Cohort-standard footer with build SHA / host / timestamp.

### Plumbing

- Path-deps on `cor24-emulator`, `cor24-assembler`, `cor24-isa` cloned alongside this repo.
- `Trunk.toml` binds dev server to `0.0.0.0:8414` so previews are reachable from another machine on the LAN.
- `scripts/build-pages.sh` builds `dist/` and rsyncs to `pages/`; `.github/workflows/pages.yml` deploys `pages/` on push to `main`.
- `pages/.nojekyll` committed so GitHub Pages serves the WASM/JS bundle directly.

### Future work

- Stage 1 will be replaced by running the SNOBOL4-based FTI-0 compiler (`sw-cor24-fortran`) inside a nested COR24 emulator, once dcftn ships a non-stub `driver.sno`. The Rust mini-compiler is a deliberate scope reduction for a working demo today.
- Variables, runtime arithmetic, control flow (`DO`, `IF`, `GOTO`), and integer I/O at runtime are out of scope for v1.
