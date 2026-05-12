# Changelog

## 2026-05-12 &mdash; Wire m4 + m5: integer PRINT, INTEGER decls, ASSIGN literal, PRINT var

dcftn shipped two more milestones on top of m3-emit-hello:

- **m4-print-int** &mdash; `PRINT *, <integer-literal>` end-to-end. Required a new runtime helper (`snobol4/runtime/putint.s`) which `scripts/fortran` splices into the emit_asm output at a `; __RUNTIME_PUTINT__` marker. emit_asm can't emit `_putint` inline because doing so would push the SNOBOL4 program past dcsno's ~233-statement static-program-size limit.
- **m5-print-var** &mdash; `INTEGER <name>` declarations, `<name> = <literal>` assignments, and `PRINT *, <name>` (integer variable).

### What changed

- `assets/` &mdash; refreshed `normalize.sno`, `classify.sno`, `emit_asm.sno` from `sw-cor24-fortran/snobol4/src/`. Added `putint.s` from `sw-cor24-fortran/snobol4/runtime/`.
- `src/compiler.rs` &mdash; after stage 1 emits assembly, splice `assets/putint.s` at the `; __RUNTIME_PUTINT__` marker. Mirrors the awk step at the end of upstream `scripts/fortran`.
- `examples/` &mdash; added `print-int.f` and `print-var.f` (the new canonical demos for m4/m5).
- `src/demos.rs` &mdash; **WORKING** now contains `hello.f`, `print-int.f`, `print-var.f` (three demos that compile + assemble + run end-to-end). **PENDING** still: `array1.f` (DIMENSION + arrays), `goto1.f` (GOTO + IF), `sum10.f` (DO loops + integer-expression ASSIGN).
- `src/help.rs` &mdash; Reference tab asset table now lists `putint.s`; Usage tab updated to enumerate which kinds emit_asm now supports.

## 2026-05-12 &mdash; Surface emit_asm `* WARN:` lines as compile errors

`emit_asm.sno` writes malformation warnings with a `*` prefix (SNOBOL4 comment convention), but COR24-asm uses `;` for comments. Those lines previously broke stage 2 with a cryptic `label must be on its own line`. Now stage 1 detects them, names the specific unsupported statements, and explains which milestone they wait on. Stage 2 is not invoked when warns are present.

## 2026-05-12 &mdash; Split demos dropdown into Works today / Awaiting upstream

Two `<optgroup>`s make the per-demo support state obvious at a glance. As dcftn ships milestones, demos graduate from PENDING to WORKING by moving one slice entry.

## 2026-05-12 &mdash; Drop Path A; wire the real FTI-0 compiler chain

dcftn shipped m2-classify and m3-emit-hello: the compiler is now a real three-phase SNOBOL4 pipeline (`normalize` &rarr; `classify` &rarr; `emit_asm`) that produces actual COR24 assembly. The Path-A short-circuit (which swapped in dcftn's hand-written `hello.s` for the canonical hello.f) is no longer needed. Stage 1 in `compiler.rs` is now three sequential `EmulatorCore` invocations mirroring `scripts/fortran` upstream.

## 2026-05-08 &mdash; Initial release: FORTRAN Hello World on COR24

First live demo of [`web-sw-cor24-fortran`](https://github.com/sw-embed/web-sw-cor24-fortran) deployed to <https://sw-embed.github.io/web-sw-cor24-fortran/>. Yew/WASM frontend that ran the early upstream toolchain in your browser. Initial architecture: nested `cor24-emulator` loaded `snobol4.lgo` and ran the `driver.sno` stub on user `.f`; a Path-A short-circuit swapped in dcftn's hand-written `hello.s` for the canonical hello.f. Multi-stage UI (Compile / Assemble / Run), editable source, demos dropdown, `[?]` Help modal. See `git log` for the rebuild history.
