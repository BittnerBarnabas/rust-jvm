package tests.arrays;

public class ArraysSetFields {
    public static void main(String... args) {
        Wrapper[] int_array_1 = new Wrapper[7];
        int_array_1[1] = new Wrapper(5);
        int_array_1[4] = new Wrapper(2);

        if (int_array_1[1].value * int_array_1[4].value != 10) {
            ((Object) null).hashCode();
        }
    }

    public static class Wrapper {
        private int value;

        public Wrapper(int value) {
            this.value = value;
        }

    }
}