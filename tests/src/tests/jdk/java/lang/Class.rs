use std::path::PathBuf;
use crate::tests::run_jvm;

#[test]
pub fn test() {
    let return_code = run_jvm(String::from("tests/java/lang/ClassTest")).expect("JVM Should've exited normally!");
    assert_eq!(0, return_code);
}