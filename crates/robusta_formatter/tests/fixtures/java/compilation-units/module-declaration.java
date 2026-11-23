@Deprecated  open  module  com.example.app {
    requires  java.base ;
    requires  transitive  java.logging ;
    requires  static  transitive  com.example.ann ;

    exports  com.example.api ;
    exports  com.example.internal  to  com.example.app , com.example.test ;
    opens  com.example.impl ;
    opens  com.example.model  to  com.fasterxml.jackson.databind ;

    uses  com.example.spi.Service ;
    provides  com.example.spi.Service  with  com.example.impl.Fast , com.example.impl.Slow ;
}
