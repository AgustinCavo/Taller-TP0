    : next1 dup 2 * ;
    : next2  next1 next1 ;
    : next4  next2 next2 ;
    : mul1 * ;
    : mul2  mul1 mul1 ;
    : mul4  mul2 mul2 ;
    1
    next4
    mul4
