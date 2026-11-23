class Outer {
    void plain( Outer this ) { }

    void annotated(@Receiver Outer this, int value) { }

    void commented(Outer /* receiver */ this, int value) { }

    class Inner {
        Inner( @Receiver Outer Outer . this ) { }

        Inner(Outer Outer.this, String firstArgumentWithALongName, String secondArgumentWithALongName) { }

        void typeAnnotated(Outer.@Receiver Inner this) { }
    }
}
