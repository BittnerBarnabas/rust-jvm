use crate::share::utilities::jvm_exception::JvmException;

pub trait Parser<T> {
    fn parse(&self, input: &str) -> Result<T, JvmException>;
}
