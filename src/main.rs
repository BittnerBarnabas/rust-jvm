use runtime::core::class_loader::{BootstrapClassLoader, ResourceLocator};
use runtime::core::context::GlobalContext;
use runtime::core::heap::heap::JvmHeap;
use std::rc::Rc;

fn main() {
    log4rs::init_file(
        "/home/barnab/CLionProjects/rust-jvm/log4rs.yml",
        Default::default(),
    )
    .unwrap();

    log::error!("abc");

    let locator = ResourceLocator::new(String::from(
        "/home/barnab/CLionProjects/rust-jvm/resources",
    ));
    let heap = Rc::new(JvmHeap::new());
    let context = Rc::new(GlobalContext::new(heap));
    let loader = Rc::new(BootstrapClassLoader::new(locator, context.clone()));
    context.set_class_loader(loader.clone());
}
