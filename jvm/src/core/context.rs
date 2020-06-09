use crate::core::class_loader::ClassLoader;
use crate::core::heap::heap::JvmHeap;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

pub struct GlobalContext {
    heap: Rc<JvmHeap>,
    class_loader: RefCell<Option<Rc<dyn ClassLoader>>>,
}

impl GlobalContext {
    pub fn new(heap: Rc<JvmHeap>) -> GlobalContext {
        GlobalContext {
            heap,
            class_loader: RefCell::new(None),
        }
    }

    pub fn heap(&self) -> Rc<JvmHeap> {
        self.heap.clone()
    }

    pub fn set_class_loader(&self, class_loader: Rc<dyn ClassLoader>) {
        self.class_loader.borrow_mut().replace(class_loader);
    }

    pub fn class_loader(&self) -> Rc<dyn ClassLoader> {
        let inner = self.class_loader.borrow().clone();
        assert!(
            inner.is_some(),
            "class_loader should be set before accessing it!"
        );
        inner.unwrap()
    }
}
