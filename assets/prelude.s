; FTI-0 runtime: _start, _halt, _putc, _puts
;
; Spliced into the SNOBOL4-emitted .s by scripts/fortran at the
; marker `; __RUNTIME_PRELUDE__` (located right after the `.text`
; directive that emit_asm.sno still emits). Moved out of the
; SNOBOL4 source to free statement budget in emit_asm.sno; the
; dcsno interpreter silently miscompiles SNOBOL4 programs whose
; static statement / label table grows past a structure-dependent
; cap around ~200-230 statements (see
; tools/briefs/dcsno-static-program-size-limit.md).
;
; Each subroutine pushes fp/r2/r1, sets fp=sp, does its work,
; restores in reverse. The 9(fp) offset on first arg is the
; documented MM-stack-frame layout (3 saved regs * 1 word + 1
; return-address slot; arg is at fp+9 after the saves).

        .globl  _start
_start:
        la      r0,_main
        jal     r1,(r0)
_halt:
        bra     _halt

        .globl  _putc
_putc:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
L1:
        la      r0,16711937
        lbu     r0,0(r0)
        push    r0
        la      r0,128
        mov     r1,r0
        pop     r0
        and     r0,r1
        ceq     r0,z
        brt     L2
        bra     L1
L2:
        lw      r0,9(fp)
        push    r0
        la      r0,16711936
        mov     r1,r0
        pop     r0
        sb      r0,0(r1)
L0:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

; _aindex(base, idx) -- returns base + (idx - 1) * 3 in r0.
; FORTRAN arrays are 1-indexed and COR24 words are 3 bytes.
; Args: base @ 12(fp), idx @ 9(fp). Used by the FTI-0 array
; load/store codegen so emit_asm.sno doesn't have to inline
; the address math for each array reference.

        .globl  _aindex
_aindex:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        lw      r0,9(fp)
        add     r0,-1
        push    r0
        la      r0,3
        mov     r1,r0
        pop     r0
        mul     r0,r1
        push    r0
        lw      r0,12(fp)
        mov     r1,r0
        pop     r0
        add     r0,r1
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .globl  _puts
_puts:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
L4:
        lw      r0,9(fp)
        lbu     r0,0(r0)
        ceq     r0,z
        brt     L5
        lw      r0,9(fp)
        lbu     r0,0(r0)
        push    r0
        la      r0,_putc
        jal     r1,(r0)
        add     sp,3
        lw      r0,9(fp)
        push    r0
        lc      r0,1
        mov     r1,r0
        pop     r0
        add     r0,r1
        sw      r0,9(fp)
        bra     L4
L5:
L3:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)
