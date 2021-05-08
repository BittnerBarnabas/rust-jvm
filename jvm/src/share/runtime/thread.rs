use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;

use crate::share::classfile::constant_pool::Qualifier;
use crate::share::runtime::api_event::ApiEvent;
use crate::share::runtime::stack_frame::{JvmStackFrame, StackFrame};
use crate::share::utilities::context::GlobalContext;
use crate::share::utilities::jvm_exception::JvmException;
use crate::share::utilities::jvm_value::JvmValue;

pub struct MainJavaThread {
    context: Arc<GlobalContext>,
}

impl MainJavaThread {
    pub fn new(context: Arc<GlobalContext>) -> MainJavaThread {
        MainJavaThread { context }
    }

    pub fn start(&self, api_event_receiver: Receiver<ApiEvent>) -> JoinHandle<Result<i32, JvmException>> {
        log::trace!("Starting MainJavaThread");
        let context = self.context.clone();
        thread::spawn(move || -> Result<i32, JvmException> {
            log::trace!("Bootstrapping classes");
            let class_loader = &context.class_loader();
            class_loader.bootstrap()?;
            loop {
                match api_event_receiver.recv().unwrap() {
                    ApiEvent::ShutDownEvent => return Ok(0),
                    ApiEvent::CallMainMethodEvent { init_class } => return MainJavaThread::call_main_method(&context, init_class)
                }
            }
        })
    }

    fn call_main_method(context: &Arc<GlobalContext>, init_class_name:String) -> Result<i32, JvmException> {
        log::trace!("Trying to look up init class {}", init_class_name);
        let class_loader = context.class_loader();
        let init_class = class_loader.load_and_init_class(&init_class_name)?;

        let main_method = class_loader.lookup_static_method(Qualifier::MethodRef {
            class_name: init_class_name.clone(),
            descriptor: String::from("([Ljava/lang/String;)V"),
            name: String::from("main"),
        })?;

        log::trace!("Executing main method of init class: {}", init_class_name);

        let frame = StackFrame::new(&context, init_class.clone());

        match frame.execute_method(main_method, Vec::new())? {
            JvmValue::Int { val } => Ok(val),
            JvmValue::Void { .. } => Ok(0),
            invalid_value => Err(JvmException::from(format!("Main method didn't return int, but: {:?}", invalid_value)))
        }
    }
}
