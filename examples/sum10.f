C     FTI-0 Example: Sum 1 to 10
      PROGRAM SUM10
      INTEGER I
      INTEGER S
      S = 0
      DO 100 I = 1, 10
      S = S + I
  100 CONTINUE
      PRINT *, S
      STOP
      END
