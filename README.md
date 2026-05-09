# web-sw-cor24-fortran

Browser-based FORTRAN hello-world demo for the
[COR24-TB](https://makerlisp.com) p-code virtual machine.

**Live demo:** <https://sw-embed.github.io/web-sw-cor24-fortran/>

## What this is

A small Yew/WASM page that embeds the COR24 emulator and runs a
pre-built `examples/hello.lgo` produced upstream by
[`sw-cor24-fortran`](https://github.com/sw-embed/sw-cor24-fortran)
(the SNOBOL4-based FTI-0 Fortran compiler). The page shows the
`hello.f` source on the left and the program's UART output on the
right.

This repo is intentionally narrow: a single hello-world demo, no
in-browser compiler, no editor. The Trunk + Yew shape is reusable for
future Fortran demos as the upstream compiler matures.

## Pipeline

```
hello.f                     <- examples/hello.f (FORTRAN source)
   |
   v  sw-cor24-fortran  (SNOBOL4)
hello.s                     <- COR24 assembly
   |
   v  cor24-asm
hello.lgo                   <- examples/hello.lgo (loadable object)
   |
   v  cor24-emulator (compiled to WASM)
"Hello, World!"             <- this page
```

## Building locally

You need [Trunk](https://trunkrs.dev/) and a Rust toolchain with the
`wasm32-unknown-unknown` target.

```bash
# Sibling path-deps (cor24-emulator + cor24-isa) must be checked out
# next to this repo. From your sw-embed/ workspace root:
[ -d sw-cor24-emulator ] || git clone <bare>/sw-cor24-emulator.git
[ -d sw-cor24-isa ]      || git clone <bare>/sw-cor24-isa.git

cd web-sw-cor24-fortran
./scripts/serve.sh       # http://127.0.0.1:8414/
```

## Deploying

```bash
./scripts/build-pages.sh   # writes to pages/
git add pages/ && git commit -m "Deploy: rebuild pages/"
git push
```

`.github/workflows/pages.yml` deploys `pages/` on push to `main`.
A `pages/.nojekyll` marker is committed so GitHub Pages serves the
WASM and JS shim files directly without Jekyll preprocessing.

## License

MIT &mdash; see [LICENSE](LICENSE).
