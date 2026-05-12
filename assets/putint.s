; FTI-0 runtime: _putint
;
; Print a 24-bit signed integer in decimal followed by NO newline
; (callers add the trailing newline if list-directed PRINT requires).
; Arg @ 9(fp). Implements decimal conversion by repeated subtraction
; since COR24 has no native div/mod.
;
; This block is spliced in by scripts/fortran after the SNOBOL4-emitted
; assembly. emit_asm.sno cannot emit _putint inline because adding
; ~70 OUTPUT statements pushes the compiler program past dcsno's
; ~233-statement static-program-size limit (see
; tools/briefs/dcsno-static-program-size-limit.md).

        .globl  _putint
_putint:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-11
        lw      r0,9(fp)
        ceq     r0,z
        brf     PINZ
        lc      r0,48
        push    r0
        la      r0,_putc
        jal     r1,(r0)
        add     sp,3
        bra     PIDONE
PINZ:
        lc      r0,0
        sw      r0,-11(fp)
PIEXT:
        lw      r0,9(fp)
        ceq     r0,z
        brt     PIPRT
        lw      r0,9(fp)
        lc      r1,0
PISUB:
        la      r2,10
        cls     r0,r2
        brt     PISUBD
        add     r0,-10
        add     r1,1
        bra     PISUB
PISUBD:
        add     r0,48
        push    r0
        lw      r2,-11(fp)
        lc      r0,-8
        add     r0,fp
        add     r0,r2
        pop     r2
        sb      r2,0(r0)
        lw      r0,-11(fp)
        add     r0,1
        sw      r0,-11(fp)
        sw      r1,9(fp)
        bra     PIEXT
PIPRT:
        lw      r0,-11(fp)
        ceq     r0,z
        brt     PIDONE
        lw      r0,-11(fp)
        add     r0,-1
        sw      r0,-11(fp)
        lc      r0,-8
        add     r0,fp
        lw      r1,-11(fp)
        add     r0,r1
        lbu     r0,0(r0)
        push    r0
        la      r0,_putc
        jal     r1,(r0)
        add     sp,3
        bra     PIPRT
PIDONE:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)
