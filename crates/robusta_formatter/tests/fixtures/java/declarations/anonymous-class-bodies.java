class Registry {
    private final Runnable task = new  Runnable ( ) {
        @Override public void run ( ) { step ( ) ; }
    } ;

    void register ( int id ) {
        Handler h = new Handler ( id ) {
            private int count = 0 ;

            @Override void handle ( ) { count ++ ; }
        } ;
        Marker m = new Marker ( ) { } ;
    }
}
