class Patterns {
    boolean check(Object o) {
        if(o instanceof @Nonnull String name)return name.length()>0;
        if(o instanceof final var value)return value!=null;
        if(o instanceof int count)return count>0;
        return o instanceof java.util.Map<String,Integer> _;
    }
}
