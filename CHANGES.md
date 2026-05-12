# Changelog

## 2026-05-12 &mdash; Drop Path A; wire the real FTI-0 compiler chain

dcftn shipped milestones m2-classify and m3-emit-hello: their compiler is now a real three-phase SNOBOL4 pipeline (`normalize` &rarr; `classify` &rarr; `emit_asm`) that produces actual COR24 assembly. The Path-A short-circuit (which swapped in dcftn's hand-written `hello.s` for the canonical hello.f) is no longer needed.

### What changed

- `src/compiler.rs` &mdash; replaced the single-shot SNOBOL4 driver invocation + Path-A fallback with three sequential `EmulatorCore` instances mirroring `scripts/fortran` upstream. Each phase loads `snobol4.lgo` via `load_lgo`, drops its `.sno` source at `0x080000` and its input at `0x090000`, then runs to halt. The output of phase N feeds phase N+1.
- `src/main.rs` &mdash; dropped the `via_path_a` state and the "Path-A short-circuit" badge. The status line and details pane now refer to the three-phase trace rather than a SNOBOL4 driver log.
- `assets/` &mdash; refreshed `snobol4.lgo` from `work/lib/cor24/`; added `normalize.sno`, `classify.sno`, `emit_asm.sno` from `sw-cor24-fortran/snobol4/src/`; dropped the stale `fortran.sno` (was just `driver.sno`'s 5-line stub) and `hello.s` (was the Path-A fixture). `examples/hello.lgo` also removed &mdash; nothing references it anymore.
- `src/demos.rs` &mdash; demo labels honest about what compiles today: only `hello.f` works end-to-end; `array1.f`, `goto1.f`, `sum10.f` exercise FTI-0 features (`INTEGER`, `DIMENSION`, `DO`, `GOTO`, `IF`, integer `PRINT`) that emit_asm doesn't support yet.
- `src/help.rs` &mdash; Reference tab now describes the three-phase chain, the asset table, and how to refresh from upstream.

### What works today

- `hello.f` &rarr; `Hello, World!` end-to-end (real compilation now, not Path A).
- The three other bundled demos run through the chain but `emit_asm.sno` doesn't yet emit code for their statement kinds. The user sees the chain produce something but `looks_like_asm` returns false; a friendly compile error explains what's missing.

### What's next upstream

dcftn is working on `feat/m4-print-int` (integer PRINT). When that ships, `sum10.f` should compile. Subsequent milestones add DO loops, GOTO/IF, INTEGER declarations, and DIMENSION/arrays. Each unlock is just a `cp` of the updated `.sno` files plus `./scripts/build-pages.sh`.

## 2026-05-08 &mdash; Initial release: FORTRAN Hello World on COR24

First live demo of [`web-sw-cor24-fortran`](https://github.com/sw-embed/web-sw-cor24-fortran) deployed to <https://sw-embed.github.io/web-sw-cor24-fortran/>. Yew/WASM frontend that ran the early upstream toolchain in your browser. Initial architecture: nested `cor24-emulator` loaded `snobol4.lgo` and ran the `driver.sno` stub on user `.f`; a Path-A short-circuit swapped in dcftn's hand-written `hello.s` for the canonical hello.f. Multi-stage UI (Compile / Assemble / Run), editable source, demos dropdown, `[?]` Help modal. See `git log` for the rebuild history.
