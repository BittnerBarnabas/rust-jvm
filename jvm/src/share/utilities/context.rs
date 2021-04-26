use crate::share::classfile::class_loader::ClassLoader;
use crate::share::memory::heap::JvmHeap;
use std::sync::{Arc, RwLock};
use crate::share::native::native_method_repo::NativeMethodRepo;

pub struct GlobalContext {
    heap: Arc<JvmHeap>,
    class_loader: RwLock<Option<Arc<dyn ClassLoader>>>,
    native_method_repo: RwLock<Option<Arc<NativeMethodRepo>>>,
}

impl GlobalContext {
    pub fn new(heap: Arc<JvmHeap>) -> GlobalContext {
        log::trace!("Initializing GlobalContext");
        GlobalContext {
            heap,
            class_loader: RwLock::new(None),
            native_method_repo: RwLock::new(None),
        }
    }

    pub fn heap(&self) -> Arc<JvmHeap> {
        self.heap.clone()
    }

    pub fn set_class_loader(&self, class_loader: Arc<dyn ClassLoader>) {
        self.class_loader.write().unwrap().replace(class_loader);
    }

    pub fn class_loader(&self) -> Arc<dyn ClassLoader> {
        self.class_loader
            .read()
            .unwrap()
            .as_ref()
            .expect("class_loader should be set before accessing it!")
            .clone()
    }

    pub fn set_native_method_repo(&self, native_method_repo: Arc<NativeMethodRepo>) {
        self.native_method_repo.write().unwrap().replace(native_method_repo);
    }

    pub fn native_method_repo(&self) -> Arc<NativeMethodRepo> {
        self.native_method_repo
            .read()
            .unwrap()
            .as_ref()
            .expect("native_method_repo should be set before accessing it!")
            .clone()
    }
}
