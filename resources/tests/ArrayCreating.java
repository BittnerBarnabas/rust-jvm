package tests;

public class ArrayCreating {

    public static void main(String... args) {
        MyClass[] myClasses = new MyClass[4];
        MyClass[] myClasses2 = new MyClass[]{
            new MyClass()
        };

        boolean a = 2 == myClasses.length;
        boolean b = 1 == myClasses2.length;
    }

    public static class MyClass {
        private int a;
    }

    public void method1(int a) {
	                    int b = a + 5;
			            }

}
