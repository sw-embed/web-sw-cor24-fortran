C     FTI-0 Example: Array initialization and access
      PROGRAM ARR1
      INTEGER I
      DIMENSION A(5)
      DO 100 I = 1, 5
      A(I) = I * 10
  100 CONTINUE
      PRINT *, A(3)
      STOP
      END
