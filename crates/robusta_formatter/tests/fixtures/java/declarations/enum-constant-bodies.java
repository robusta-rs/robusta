enum  Op {
    PLUS ( "+" ) { int apply ( int a , int b ) { return a + b ; } } ,
    TIMES {
        @Override int apply ( int a , int b ) { return a * b ; }
    } ,
    UNIT ( ) { } ;

    abstract int apply ( int a , int b ) ;
}
