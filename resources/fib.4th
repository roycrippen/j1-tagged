decimal

: fibonacci dup 2 < if drop 1 else dup 2 - recurse swap 1 - recurse + then ;
: fibnums for i fibonacci u. next ;

14 fibonacci .
( should be -> 610 )

6 fibnums .
( should be -> 13 8 5 3 2 1 1 0 )


