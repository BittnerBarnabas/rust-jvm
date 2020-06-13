use jvm::share::classfile::class_loader::{BootstrapClassLoader, ResourceLocator};
use jvm::share::memory::heap::JvmHeap;
use jvm::share::runtime::thread::MainJavaThread;
use jvm::share::utilities::context::GlobalContext;
use std::rc::Rc;
use std::sync::Arc;

fn main() {
    log4rs::init_file(
        "/home/barnab/CLionProjects/rust-jvm/log4rs.yml",
        Default::default(),
    )
    .unwrap();

    let locator = ResourceLocator::new(String::from(
        "/home/barnab/CLionProjects/rust-jvm/resources",
    ));
    let heap = Arc::new(JvmHeap::new());
    let context = Arc::new(GlobalContext::new(heap));
    let loader = Arc::new(BootstrapClassLoader::new(locator, context.clone()));
    context.set_class_loader(loader.clone());

    let main_thread = MainJavaThread::new(context.clone());
    let handle = main_thread.start();
    handle.join().unwrap();
    println!("ABC")
}
