# Changelog

## 2026-05-13 &mdash; Wire m6-m9: all four pending demos now compile

Upstream burst: dcftn shipped four milestones since yesterday's m5 refresh:

- **m6-assign-expr** &mdash; binary `+ - *` in `<name> = <expr>`. Plus a `runtime/prelude.s` split (the `_start / _halt / _putc / _puts` block moved out of `emit_asm.sno`'s inline OUTPUTs to keep the SNOBOL4 source under dcsno's ~230-statement static-program-size limit). Spliced into the assembly via a `; __RUNTIME_PRELUDE__` marker, same pattern as putint.
- **m7-goto** &mdash; statement labels, `GOTO <label>`, `IF (expr) GOTO <label>`. `goto1.f` now compiles + assembles + runs end-to-end.
- **m8-do-loop** &mdash; `DO <label> <var> = <start>, <stop>` / `CONTINUE`. `sum10.f` now works.
- **m9-array** &mdash; `DIMENSION A(N)` reserves N words; array indexing in LHS, RHS, and PRINT. `array1.f` now works. New runtime helper `_aindex(base, idx)` is emitted inline.

### What changed here

- `assets/` &mdash; refreshed `normalize.sno`, `classify.sno`, `emit_asm.sno` from upstream `origin/dev` (emit_asm grew from 180 to 266 lines). Refreshed `putint.s`. **Added `prelude.s`** as a second spliced runtime. Refreshed `snobol4.lgo` (dcsno reshipped between m5 and m9).
- `src/compiler.rs` &mdash; `splice_putint` is now `splice_runtimes` and handles both `; __RUNTIME_PRELUDE__` and `; __RUNTIME_PUTINT__` markers in a single pass.
- `examples/` &mdash; added `add.f` (the canonical m6 demo: `A = 7; B = 13; C = A + B; PRINT *, C`).
- `src/demos.rs` &mdash; all seven `.f` files in WORKING, ordered by milestone. PENDING is now empty (kept the slice for future use as new milestones arrive).
- `src/main.rs` &mdash; demo dropdown collapses the "Awaiting upstream milestone" optgroup when PENDING is empty.
- `src/help.rs` &mdash; Reference asset table lists `prelude.s`; Usage tab enumerates m3-m9 features and notes the seven demos all compile.

### Demos that compile today

| File | Milestone | Exercises |
|---|---|---|
| `hello.f` | m3 | PROGRAM / STOP / END + `PRINT *, 'string'` |
| `print-int.f` | m4 | `PRINT *, 42` (integer literal) |
| `print-var.f` | m5 | `INTEGER X`, `X = 42`, `PRINT *, X` |
| `add.f` | m6 | `C = A + B` |
| `goto1.f` | m7 | label `100` + `GOTO 100` + `IF (I - 6) GOTO 100` |
| `sum10.f` | m8 | `DO 100 I = 1, 10` + `CONTINUE` |
| `array1.f` | m9 | `DIMENSION A(5)` + `A(I) = I * 10` + `PRINT *, A(3)` |

## 2026-05-13 &mdash; Detect `; WARN:` (dcftn fixed `*` &rarr; `;`)

`emit_asm.sno` switched the warning-comment prefix from `*` to `;` so partial assembly stays well-formed. Detector updated to accept both forms.

## 2026-05-12 &mdash; Wire m4 + m5: integer PRINT, INTEGER decls, PRINT var

`emit_asm.sno` shipped `PRINT *, <int literal>`, `INTEGER <name>`, `<name> = <literal>`, `PRINT *, <name>`. Plus the first runtime splice: `putint.s` at the `; __RUNTIME_PUTINT__` marker. Two new demos: `print-int.f`, `print-var.f`.

## 2026-05-12 &mdash; Surface `* WARN:` as compile error; split demos dropdown

Surfaces malformed-input warnings as friendly compile-stage errors instead of letting them break stage 2. Demos dropdown split into "Works today" / "Awaiting upstream milestone" optgroups.

## 2026-05-12 &mdash; Drop Path A; wire the real FTI-0 compiler chain

dcftn shipped m2-classify and m3-emit-hello: the compiler is now a real three-phase SNOBOL4 pipeline (`normalize` &rarr; `classify` &rarr; `emit_asm`). The Path-A short-circuit retired.

## 2026-05-08 &mdash; Initial release: FORTRAN Hello World on COR24

First live demo of [`web-sw-cor24-fortran`](https://github.com/sw-embed/web-sw-cor24-fortran) deployed to <https://sw-embed.github.io/web-sw-cor24-fortran/>. Yew/WASM, three-stage UI (Compile / Assemble / Run), demos dropdown, `[?]` modal, cohort footer.
