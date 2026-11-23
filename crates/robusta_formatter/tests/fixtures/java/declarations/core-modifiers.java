strictfp  class Mods {
    transient   volatile int flags ;
    native synchronized void ping( ) ;

    strictfp double ratio( ) { return 1.0d ; }
}
