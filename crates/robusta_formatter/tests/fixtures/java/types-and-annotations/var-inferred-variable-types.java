class Inference {
    void run(Items items) {
        var  total=count( ) ;
        final  var name = load( ) ;
        for ( var item : items ) { use( item ) ; }
        try ( var in = open( ) ; final var out = create( ) ) { copy( in , out ) ; }
        mapper = ( var left , @Nonnull var right ) -> left ;
    }
}
