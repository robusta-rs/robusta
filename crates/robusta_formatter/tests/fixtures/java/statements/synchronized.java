class Locks {
    synchronized void reset( ) {
        synchronized(this.lock){
            count++;
        }
        synchronized ( Registry . global ( ) ) { waitFor ( ready ) ; }
    }
}
