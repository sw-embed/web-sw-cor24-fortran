# Changelog

## 2026-05-08 — Initial release: FORTRAN Hello World on COR24

First live demo of the [`web-sw-cor24-fortran`](https://github.com/sw-embed/web-sw-cor24-fortran) repo, deployed to <https://sw-embed.github.io/web-sw-cor24-fortran/>. Yew/WASM frontend that embeds dcftn's pre-built `examples/hello.lgo` (Path A: hand-written hello.s assembled to .lgo, shipped by `sw-cor24-fortran`) and runs it inside `cor24-emulator` compiled to WASM.

### What's on the page

- The canonical `examples/hello.f` source on the left, read-only.
- The program's UART output on the right, populated as the embedded emulator runs.
- A **Run / Restart** button to replay; auto-runs on page load.

This repo is intentionally narrow: a thin demo shell over the upstream Fortran compiler's output. No in-browser compilation, no editor, no multi-demo dropdown — those belong to the upstream sw-cor24-fortran repo (and to a future "richer demo" saga once dcftn's SNOBOL4-based compiler is far enough along to consume in-browser).

### Plumbing

- Yew + Trunk, single binary (no `[lib]`).
- `cor24-emulator` is the only runtime path-dep (cloned alongside this repo). The `.lgo` is loaded via `EmulatorCore::load_lgo`; no in-browser assembly.
- `Trunk.toml` binds the dev server to `0.0.0.0:8414` so previews are reachable from another machine on the LAN.
- `scripts/build-pages.sh` builds `dist/` and rsyncs to `pages/`; `.github/workflows/pages.yml` deploys `pages/` on push to `main`.
- `pages/.nojekyll` committed so GitHub Pages serves the WASM/JS bundle directly.

### Archive

`docs/exploration/` contains an off-brief earlier draft (Rust Fortran mini-compiler + editor + multi-stage UI). Not shipped in v1. Kept as a reference for a possible future saga that upgrades this demo to consume dcftn's SNOBOL4-based compiler running in a nested COR24 emulator.

### Upstream chain

```
hello.f                     <- examples/hello.f (FORTRAN source)
   |
   v  sw-cor24-fortran  (Path A: hand-written hello.s, shipped by dcftn)
hello.s                     <- COR24 assembly
   |
   v  cor24-asm
hello.lgo                   <- examples/hello.lgo  ← dwftn embeds this
   |
   v  cor24-emulator (compiled to WASM)
"Hello, World!"             <- this page
```
