@Deprecated  public class Config {
    @Deprecated@SuppressWarnings( "unchecked" )   private int a;

    @Partial @Mock DataLoader loader;

    @Named( value="x" , scopes={@Scope( SINGLETON ),@Scope( REQUEST ) , } )
    private String b;

    @Override public String toString( ) { return "x" ; }

    void f(@Nullable String s, final @Foo( 1 ) int n) {
        @Foo final int x = 1 ;
        @Bar( 2 ) int y = 2 ;
        for (@Each int i = 0; i < 3; i++) g( ) ;
        for (@Each final String t : names) g();
    }

    @java.lang.Deprecated @Beta( ) void g() { }

    @A @B Runnable r = () -> { foo(); };
    @A @B int x = // c
        1;
    @A @B String s = """
        text
        """;
}
