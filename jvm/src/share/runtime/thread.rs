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
            context.class_loader().bootstrap()
        })
    }
}
