use crate::share::classfile::class_loader::BootstrapClassLoader;
use crate::share::classfile::constant_pool::Qualifier;
use crate::share::runtime::stack_frame::{JvmStackFrame, StackFrame};
use crate::share::utilities::context::GlobalContext;
use crate::share::utilities::jvm_exception::JvmException;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

pub struct MainJavaThread {
    context: Arc<GlobalContext>,
}

impl MainJavaThread {
    pub fn new(context: Arc<GlobalContext>) -> MainJavaThread {
        MainJavaThread { context }
    }

    pub fn start(&self) -> JoinHandle<Result<(), JvmException>> {
        log::trace!("Starting MainJavaThread");
        let context = self.context.clone();
        thread::spawn(move || -> Result<(), JvmException> {
            log::trace!("Bootstrapping classes");
            let class_loader = context.class_loader();
            class_loader.bootstrap()?;

            MainJavaThread::call_main_method(&context)
        })
    }

    fn call_main_method(context: &Arc<GlobalContext>) -> Result<(), JvmException> {
        let class_loader = context.class_loader();

        let init_class_name = String::from("tests/ArrayCreating");
        let init_class = class_loader.load_and_init_class(&init_class_name)?;

        let main_method = class_loader.lookup_static_method(Qualifier::MethodRef {
            class_name: init_class_name.clone(),
            descriptor: String::from("([Ljava/lang/String;)V"),
            name: String::from("main"),
        })?;

        log::trace!("Executing main method of init class: {}", init_class_name);

        let frame = StackFrame::new(&context, init_class.clone());
        frame.execute_method(main_method, Vec::new())?;
        Ok(())
    }
}
