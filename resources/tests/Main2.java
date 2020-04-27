package mypack;

public class Main2 implements MyInterface<Object> {
	private int a = 2;
	public int b = 	    super.hashCode();
	private static final char ch = 'c';
	private static int myInt = getInt();

	public static void main(String ... args) {
	    Main2 myMainObject = new Main2();
		int i = 13;
		int j = myMainObject.getB();
		int k = i + j;
	}

	public static int getInt() {
		return 6;
	}

	public int getB() {
	    return b;
	}
}
