pub mod Symbols {
    #![allow(non_upper_case_globals)]
    lazy_static::lazy_static! {
        pub static ref java_lang_Object: String = String::from("java/lang/Object");
        pub static ref java_lang_Object_registerNatives: String = String::from("java/lang/Object_registerNatives()V");
        pub static ref java_lang_Object_hashCode: String = String::from("java/lang/Object_hashCode()I");

        pub static ref java_lang_Class_registerNatives: String = String::from("java/lang/Class_registerNatives()V");

        pub static ref java_lang_String: String = String::from("java/lang/String");
        pub static ref java_lang_Class: String = String::from("java/lang/Class");
        pub static ref java_lang_Throwable: String = String::from("java/lang/Throwable");
    }
}
