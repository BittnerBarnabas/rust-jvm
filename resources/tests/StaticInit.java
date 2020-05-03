
public class StaticInit {

    private static final int a = getA();
    private static final int b;
    static {
        b = 5;
    }

    private static int getA() {
        return 25;
    }

}