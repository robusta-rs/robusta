class Registry {
    static final java.util.Map<String, String> DEFAULTS = new java.util.HashMap<>();

    static{
        DEFAULTS.put("a","1");
        DEFAULTS.put("b","2");
    }

    int count;

    {count=0;}

    Registry() {}
}
