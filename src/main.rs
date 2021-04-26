use jvm::share::classfile::class_loader::{BootstrapClassLoader, ResourceLocator};
use jvm::share::memory::heap::JvmHeap;
use jvm::share::runtime::thread::MainJavaThread;
use jvm::share::utilities::context::GlobalContext;
use std::rc::Rc;
use std::sync::Arc;
use std::borrow::{Borrow, BorrowMut};
use jvm::share::native::native_method_repo::NativeMethodRepo;

fn main() {
    log4rs::init_file(
        "/home/barnab/projects/rust-jvm/log4rs.yml",
        Default::default(),
    )
    .unwrap();

    let locator = ResourceLocator::new(String::from(
        "/home/barnab/projects/rust-jvm/resources",
    ));
    let heap = Arc::new(JvmHeap::new());
    let context = Arc::new(GlobalContext::new(heap));
    let loader = Arc::new(BootstrapClassLoader::new(locator, context.clone()));
    context.set_class_loader(loader);

    let native_method_repo = Arc::new(NativeMethodRepo::new());
    context.set_native_method_repo(native_method_repo);

    let main_thread = MainJavaThread::new(context.clone());
    let handle = main_thread.start();
    let result = handle.join().unwrap();
    result.expect("JVM didn't exit correctly");
    println!("ABC")
}
