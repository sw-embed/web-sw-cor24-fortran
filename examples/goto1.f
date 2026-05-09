C     FTI-0 Example: Goto loop counting to 5
      PROGRAM COUNT
      INTEGER I
      I = 1
  100 PRINT *, I
      I = I + 1
      IF (I - 6) GOTO 100
      STOP
      END
