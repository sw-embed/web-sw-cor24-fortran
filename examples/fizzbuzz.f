C     FTI-0 Example: FizzBuzz 1..15
C     COR24 has no div/mod, so we track multiples by counter
C     reset: M3 ticks up to 3 then resets; M5 ticks up to 5.
      PROGRAM FB
      INTEGER I
      INTEGER M3
      INTEGER M5
      M3 = 0
      M5 = 0
      DO 100 I = 1, 15
      M3 = M3 + 1
      M5 = M5 + 1
      IF (M3 - 3) GOTO 200
      IF (M5 - 5) GOTO 300
      PRINT *, 'FizzBuzz'
      M3 = 0
      M5 = 0
      GOTO 100
  300 PRINT *, 'Fizz'
      M3 = 0
      GOTO 100
  200 IF (M5 - 5) GOTO 400
      PRINT *, 'Buzz'
      M5 = 0
      GOTO 100
  400 PRINT *, I
  100 CONTINUE
      STOP
      END
