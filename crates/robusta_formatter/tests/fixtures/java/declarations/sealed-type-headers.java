public  sealed  class Shape permits Circle , geometry . Square {
    int sides ;
}
sealed interface Node extends Base permits Leaf , tree . Branch { }
non-sealed class FreeShape extends Shape { }
class Uses { sealed value ; }
