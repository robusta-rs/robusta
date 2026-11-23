class Checks {
    void require(int count,String name) {
        assert count>0;
        assert name!=null:"name";
        if(count>10)assert name.length()>0:name;
    }
}
