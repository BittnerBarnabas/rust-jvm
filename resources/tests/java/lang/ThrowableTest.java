package tests.java.lang;

public class ThrowableTest {
    public static void main(String... args) throws Throwable {
        method1();
    }

    private static int method1() throws Throwable {
        int a = 6 + 7;
        a += method2();
        return a;
    }

    private static int method2() throws Throwable {
        throw new Throwable();
    }
}
