: NOT  ( n -- 0 | 1 )
  DUP 0= IF DROP 1 ELSE DROP 0 THEN ;

: GREATER ( n1 n2 -- 0 | 1 )
  > if 1 else 0 then ;

: ROLL ( n1 n2 -- )
  2DUP
  0 MAX
  DEPTH ROT MAX 1- MIN
  0 DO
    1- ROLL
    LOOP DROP ;


( THIS DOES NOT WORK )
: INTEGER-INPUT ( "number" -- n )
  0. parse-word >number ( d c-addr u )
    abort" NaN" drop
    abort" single-cell number expected, recieved a double instead " drop ;