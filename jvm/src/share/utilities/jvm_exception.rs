#[derive(Debug, PartialEq)]
pub struct JvmException {
    message: Option<String>,
}

impl JvmException {
    pub fn new() -> JvmException {
        JvmException { message: None }
    }
}

impl From<String> for JvmException {
    fn from(message: String) -> Self {
        JvmException {
            message: Some(message),
        }
    }
}

impl From<&'static str> for JvmException {
    fn from(str: &'static str) -> Self {
        JvmException::from(format!("{}", str))
    }
}
