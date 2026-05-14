C     FTI-0 Example: iterative fibonacci, prints fib(11) = 89
      PROGRAM FIB
      INTEGER A
      INTEGER B
      INTEGER C
      INTEGER I
      A = 0
      B = 1
      DO 100 I = 1, 10
      C = A + B
      A = B
      B = C
  100 CONTINUE
      PRINT *, B
      STOP
      END
