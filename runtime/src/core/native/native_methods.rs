use crate::core::class_loader::ClassLoader;
use crate::core::context::GlobalContext;
use crate::core::jvm_exception::JvmException;
use crate::core::jvm_value::JvmValue;
use crate::core::klass::klass::Klass;
use crate::core::klass::method::MethodInfo;
use std::rc::Rc;
use utils::ResultIterator;

pub struct NativeMethodArgs<'a> {
    current_class: &'a Klass,
    context: &'a GlobalContext,
    java_args: Vec<JvmValue>,
}

impl<'a> NativeMethodArgs<'a> {
    pub fn new<'b>(class: &'b Klass, context: &'b GlobalContext) -> NativeMethodArgs<'b> {
        NativeMethodArgs {
            current_class: class,
            context,
            java_args: Vec::new(),
        }
    }
}

pub type NativeMethod = fn(NativeMethodArgs) -> Result<JvmValue, JvmException>;

pub fn hash_code(args: NativeMethodArgs) -> Result<JvmValue, JvmException> {
    //TODO Implement this
    Ok(JvmValue::Int { val: 1 })
}

pub fn register_natives(args: NativeMethodArgs) -> Result<JvmValue, JvmException> {
    log::trace!(
        "register_natives called on class: {}",
        args.current_class.qualified_name()
    );

    let this = args.current_class;
    this.methods()
        .iter()
        .filter(|m| m.is_native() && m.native_method().is_none())
        .map(|unbound_native_method| set_native_method(unbound_native_method))
        .collect_to_result()?;

    Ok(JvmValue::Void {})
}

fn set_native_method(unbound_native_method: &Rc<MethodInfo>) -> Result<(), JvmException> {
    log::trace!(
        "Setting native method for: {}",
        unbound_native_method.name_desc()
    );
    //TODO properly implement this
    Ok(())
}
