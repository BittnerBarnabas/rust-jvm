use std::sync::Arc;
use std::sync::mpsc::{channel, Sender};

use crate::share::classfile::class_loader::{BootstrapClassLoader, ResourceLocator};
use crate::share::memory::heap::JvmHeap;
use crate::share::native::native_method_repo::NativeMethodRepo;
use crate::share::runtime::api_event::ApiEvent;
use crate::share::runtime::thread::MainJavaThread;
use crate::share::utilities::context::GlobalContext;
use std::thread::JoinHandle;
use crate::share::utilities::jvm_exception::JvmException;

pub fn init_jvm() -> impl JvmApi {
    let locator = ResourceLocator::new(String::from(
        "/home/barnab/projects/rust-jvm/resources",
    ));
    let heap = Arc::new(JvmHeap::new());
    let context = Arc::new(GlobalContext::new(heap));
    let loader = Arc::new(BootstrapClassLoader::new(locator, context.clone()));
    context.set_class_loader(loader);

    let native_method_repo = Arc::new(NativeMethodRepo::new());
    context.set_native_method_repo(native_method_repo);

    let (sender, receiver) = channel::<ApiEvent>();

    let main_thread = MainJavaThread::new(context.clone());
    let handle = main_thread.start(receiver);

    JvmApiImpl::new(sender, handle)
}

struct JvmApiImpl {
    api_event_sender: Sender<ApiEvent>,
    jvm_handle: Option<JoinHandle<Result<i32, JvmException>>>,
}

impl JvmApiImpl {
    fn new(api_event_sender: Sender<ApiEvent>, jvm_handle: JoinHandle<Result<i32, JvmException>>) -> Self {
        JvmApiImpl { api_event_sender, jvm_handle: Some(jvm_handle) }
    }
}

impl JvmApi for JvmApiImpl {
    fn shutdown(&mut self) -> Result<i32, JvmException> {
        self.api_event_sender.send(ApiEvent::ShutDownEvent).unwrap();
        self.jvm_handle.take().map(
            |main_thread| main_thread.join().unwrap()
        ).unwrap()
    }

    fn call_main_method(&mut self, init_class_name: String) -> Result<i32, JvmException> {
        self.api_event_sender.send(ApiEvent::CallMainMethodEvent { init_class: init_class_name }).unwrap();
        self.jvm_handle.take().map(
            |main_thread| main_thread.join().unwrap()
        ).unwrap()
    }
}


pub trait JvmApi {
    fn shutdown(&mut self) -> Result<i32, JvmException>;
    fn call_main_method(&mut self, init_class_name: String) -> Result<i32, JvmException>;
}