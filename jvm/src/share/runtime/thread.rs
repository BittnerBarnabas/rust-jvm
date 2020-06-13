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
        let context = self.context.clone();
        thread::spawn(move || {
            context.class_loader().bootstrap();
        })
    }
}
