class Files {
    void copy(Path src, Path dst) {
        try ( InputStream in = open( src ) ; OutputStream out = create( dst ) ){
            transfer( in , out ) ;
        }
        try(lock; this . other ; factory ( ) . owned ;){work();}finally{unlock();}
        try (VeryLongResourceTypeName firstResource = acquireTheFirstResource(context); AnotherLongType secondResource = acquireTheSecondResource(context)) {
            process();
        }
    }
}
