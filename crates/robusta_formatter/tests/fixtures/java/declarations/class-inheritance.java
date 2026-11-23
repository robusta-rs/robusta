public  class Service < T , H extends api.Handler < ? super Event > & java.io.Closeable >
extends base.AbstractService < T >
implements api.Registry < H > {
    int value ;
}
