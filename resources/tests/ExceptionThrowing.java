public class ExceptionThrowing {

    public static void main(String... args) {
        ExceptionThrowing main = new ExceptionThrowing();
        main.throwException();
    }

    public void throwException() {
        throw new RuntimeException();
    }
}