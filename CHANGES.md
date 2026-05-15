# Changelog

## 2026-05-14 &mdash; Drop runtime splice; migrate to new dcsno addresses

dcftn shipped **m13-inline-runtime** &mdash; the `_start / _halt / _putc / _puts / _putint / _aindex` runtime support routines now live inline in `emit_asm.sno` again, no longer extracted to `prelude.s` / `putint.s` files spliced post-emit. This was made possible by dcsno's `pr/cap-and-pattern-fixes` which bumped internal buffers (EPSLOTS 8&rarr;16&rarr;32, PSTK_DEPTH 16&rarr;32) and grew the interpreter image from 93 KB to 1.4 MB &mdash; emit_asm.sno fits even at 18 KB+ now.

Same release also includes dcsno's load-address migration (`pr/cap-and-pattern-fixes` + `pr/load-addr-compat`): compiler source moves from `0x080000` to `0xE0000`, input data from `0x090000` to `0xF0000`. dcftn shipped **m12-snobol4-addrs** updating `scripts/fortran` to match.

### What changed here

- `assets/`: refreshed `snobol4.lgo` (now 1.4 MB) and `{normalize,classify,emit_asm}.sno` from the canonical install at `work/lib/cor24/`. **Dropped `prelude.s` and `putint.s`** &mdash; no longer needed.
- `src/compiler.rs`:
  - Removed `splice_runtimes`, `PRELUDE_RUNTIME`, `PUTINT_RUNTIME`, `PRELUDE_MARKER`, `PUTINT_MARKER`.
  - `PROGRAM_LOAD_ADDR` `0x080000` &rarr; `0x0E0000`.
  - `INPUT_LOAD_ADDR` `0x090000` &rarr; `0x0F0000`.
  - Module doc rewritten to reflect the new architecture.
- `src/help.rs`: Reference asset table no longer lists prelude/putint. Refresh procedure updated.
- `CLAUDE.md`: asset table + load-address note updated; `dcsno-lift-source-byte-cap` brief I drafted is now effectively resolved (workaround retired).

### Verified end-to-end

All ten demos pass through `cor24-asm` + `cor24-emu` via the upstream `snobol4` chain with the new addresses + inlined runtime:

| Demo | Output |
|---|---|
| `hello.f` | `Hello, World!` |
| `print-int.f` | `42` |
| `print-var.f` | `42` |
| `add.f` | `20` (7+13) |
| `goto1.f` | `1, 2, 3, 4, 5` |
| `sum10.f` | `55` |
| `array1.f` | `30` (A(3) = 3*10) |
| `factorial.f` | `120` |
| `fibonacci.f` | `89` |
| `fizzbuzz.f` | `1, 2, Fizz, ..., 14, FizzBuzz` |

## 2026-05-13 &mdash; Wire m10 + m11: identifier validation, three classic demos

Picked up dcftn `m10-pattern-anchor` (KINT identifier validation, closes the brief I drafted) and `m11-demos` (factorial / fibonacci / fizzbuzz).

## 2026-05-13 &mdash; Wire m6-m9: all four originally pending demos now compile

`m6-assign-expr` (binary `+ - *`), `m7-goto` (labels + GOTO + IF), `m8-do-loop` (DO / CONTINUE), `m9-array` (DIMENSION + indexing). Also introduced the runtime/prelude.s split that the m13-inline-runtime saga has now retired.

## 2026-05-13 &mdash; Detect `; WARN:` (dcftn fixed `*` &rarr; `;`)

`emit_asm.sno` switched the warning-comment prefix from `*` to `;` so partial assembly stays well-formed. Detector updated to accept both forms.

## 2026-05-12 &mdash; Wire m4 + m5: integer PRINT, INTEGER decls, PRINT var

First splice: `putint.s` at `; __RUNTIME_PUTINT__`. Two new demos: `print-int.f`, `print-var.f`. (Splice retired today via m13.)

## 2026-05-12 &mdash; Surface `* WARN:` as compile error; split demos dropdown

Friendly compile-stage errors for unsupported statements; demos dropdown split into "Works today" / "Awaiting upstream milestone" optgroups. (PENDING optgroup hidden today since all demos work.)

## 2026-05-12 &mdash; Drop Path A; wire the real FTI-0 compiler chain

dcftn shipped m2-classify and m3-emit-hello: the compiler became a real three-phase SNOBOL4 pipeline. Path-A short-circuit retired.

## 2026-05-08 &mdash; Initial release: FORTRAN Hello World on COR24

First live demo of [`web-sw-cor24-fortran`](https://github.com/sw-embed/web-sw-cor24-fortran) deployed to <https://sw-embed.github.io/web-sw-cor24-fortran/>.
