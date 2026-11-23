class RecordPatterns {
    boolean check(Object value) {
        if(value instanceof Box<Point>( Point( var x , _ ) ))return x>0;
        return value instanceof Empty( );
    }

    Object run(Object value) {
        return switch(value){case Box<Point>(Point(int x,_)) when x>0->x;case Empty()->null;default->value;};
    }
}
