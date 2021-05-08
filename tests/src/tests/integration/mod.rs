use crate::tests::run_jvm;

#[test]
pub fn arrays_set_fields() {
    let return_code = run_jvm(String::from("tests/arrays/ArraysSetFields")).expect("JVM Should've exited normally!");
    assert_eq!(0, return_code);
}