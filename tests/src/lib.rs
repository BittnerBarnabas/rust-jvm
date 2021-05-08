#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use jvm::api::jvm_api;
    use jvm::api::jvm_api::JvmApi;
    use jvm::share::utilities::jvm_exception::JvmException;

    mod jdk;
    mod integration;

    fn run_jvm(init_class_name: String) -> Result<i32, JvmException> {
        log4rs::init_file(
            "/home/barnab/projects/rust-jvm/log4rs.yml",
            Default::default(),
        )
            .unwrap();

        let mut jvm = jvm_api::init_jvm();
        jvm.call_main_method(init_class_name)
    }
}
