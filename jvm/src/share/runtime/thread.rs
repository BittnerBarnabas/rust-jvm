use crate::share::utilities::context::GlobalContext;
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

    pub fn start(&self) -> JoinHandle<()> {
        log::trace!("Starting MainJavaThread");
        let context = self.context.clone();
        thread::spawn(move || {
            log::trace!("Bootstrapping classes");
            context.class_loader().bootstrap();
        })
    }
}
