pub struct JvmException {
    message: Option<String>,
}

impl JvmException {
    pub fn new() -> JvmException {
        JvmException { message: None }
    }

    pub fn from_string(message: String) -> JvmException {
        JvmException {
            message: Some(message),
        }
    }

    pub fn from_str(str: &str) -> JvmException {
        JvmException::from_string(format!("{}", str))
    }
}
