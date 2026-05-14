C     FTI-0 Example: 5 factorial via DO loop
      PROGRAM FACT
      INTEGER I
      INTEGER R
      R = 1
      DO 100 I = 1, 5
      R = R * I
  100 CONTINUE
      PRINT *, R
      STOP
      END
