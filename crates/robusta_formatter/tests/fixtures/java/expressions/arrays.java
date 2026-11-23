class Buffers {
    private static final int [ ] SIZES = { 8 , 16 , 32 , } ;
    private final byte [ ] [ ] pool=new byte [ 4 ] [ ] ;

    int [ ] [ ] table() {
        int [ ] [ ] table = new int [ ] [ ] { { 1 , 2 } , { 3 } , { } } ;
        table [ 0 ] [ 1 ] = SIZES [ table.length - 1 ] ;
        return table ;
    }

    byte [ ] slot(int i) {
        pool [ i ] = new byte [ SIZES [ i ] ] ;
        return pool [ i ] ;
    }
}
