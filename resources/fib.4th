decimal

: fib dup 2 < if drop 1 else dup 2 - recurse swap 1 - recurse + then ;
: fibs for i fib u. next ;

14 fib .
( should be -> 610 )

6 fibs .
( should be -> 13 8 5 3 2 1 1 0 )


