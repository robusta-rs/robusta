class  Greeter {
    private final String greeting="hi" ;
    Greeter( String ignored )  {
        this( ignored , 0 ) ;
        String unused = ignored ; // keep
    }
    < T > Greeter(T ignored ) {
        super( ) ;
        return ;
    }


    String greet(String name,final String suffix)throws a.StaleException,IllegalStateException {

        String message = greeting;
        return  message ;
    }
    static < T extends CharSequence > T identity(final T value)throws E{
        return value ;
    }
    abstract int count( ) ;
    void reset(){ }
}
