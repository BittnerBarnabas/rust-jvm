use crate::share::classfile::class_loader::ClassLoader;
use crate::share::memory::heap::JvmHeap;
use std::sync::{Arc, RwLock};

pub struct GlobalContext {
    heap: Arc<JvmHeap>,
    class_loader: Arc<RwLock<Option<Arc<dyn ClassLoader>>>>,
}

impl GlobalContext {
    pub fn new(heap: Arc<JvmHeap>) -> GlobalContext {
        log::trace!("Initializing GlobalContext");
        GlobalContext {
            heap,
            class_loader: Arc::new(RwLock::new(None)),
        }
    }

    pub fn heap(&self) -> Arc<JvmHeap> {
        self.heap.clone()
    }

    pub fn set_class_loader(&self, class_loader: Arc<dyn ClassLoader>) {
        self.class_loader.write().unwrap().replace(class_loader);
    }

    pub fn class_loader(&self) -> Arc<dyn ClassLoader> {
        let inner = self.class_loader.read().unwrap().clone();
        assert!(
            inner.is_some(),
            "class_loader should be set before accessing it!"
        );
        inner.unwrap()
    }
}
