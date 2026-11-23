class Errors {
    void fail(String message) {
        if(message==null)throw new IllegalArgumentException("message");
        throw new IllegalStateException( message + ".failed" );
    }
}
