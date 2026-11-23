package  com.example ;

@Doc public  @interface Route {
    String value ( ) default "/" ;
    int [ ] codes( ) default { 200 , 201 , } ;
    String legacy ( ) [ ] default { "old" , } ;
    String annotated ( ) @Dim [ ] ;
    Class<?> type( ) ;
    int LIMIT=64 ;
    @interface Nested { String name( ) ; }
    enum Mode { GET , POST }
}
