#[cfg(test)]
mod tests {
    use runtime::core::class_loader::{BootstrapClassLoader, ClassLoader, ResourceLocator};
    use runtime::core::context::GlobalContext;
    use runtime::core::heap::heap::JvmHeap;
    use runtime::core::jvm_exception::JvmException;
    use runtime::core::jvm_value::JvmValue;
    use runtime::core::klass::constant_pool::Qualifier;
    use runtime::core::klass::klass::Klass;
    use runtime::core::native::native_method_repo::NativeMethodRepo;
    use runtime::core::stack_frame::{JvmStackFrame, StackFrame};
    use std::rc::Rc;

    // use runtime::core::class_loader::ClassLoader;
    // use runtime::core::class_parser::ClassParser;
    // use runtime::core::heap::heap::JvmHeap;
    // use runtime::core::jvm_exception::JvmException;
    // use runtime::core::jvm_value::JvmValue;
    // use runtime::core::klass::constant_pool::Qualifier;
    // use runtime::core::klass::klass::Klass;
    // use runtime::core::stack_frame::StackFrame;
    // use runtime::core::{interpreter, opcode};
    // use std::rc::Rc;
    //
    // #[test]
    // pub fn storing_and_adding_local_integers() {
    //     let opcodes: Vec<u8> = vec![
    //         opcode::BIPUSH,
    //         0x10,
    //         opcode::ISTORE_1,
    //         opcode::ICONST_4,
    //         opcode::ISTORE,
    //         0x05,
    //         opcode::ILOAD_1,
    //         opcode::ILOAD,
    //         0x05,
    //         opcode::IADD,
    //         opcode::ISTORE_3,
    //         opcode::ILOAD_3,
    //         opcode::IRETURN,
    //     ];
    //
    //     // let frame = StackFrame::new();
    //     // let interpreter_result = interpreter::interpret(&frame, &opcodes).ok();
    //     // assert_eq!(interpreter_result, Some(JvmValue::Int { val: 20 }));
    // }
    //
    // #[test]
    // pub fn test() {
    //     let native_method_repo = Rc::new(NativeMethodRepo::new());
    //     let heap = Rc::new(JvmHeap::new());
    //     let class_loader = ClassLoader::new(heap.clone(), native_method_repo.clone());
    //     let result = class_loader.bootstrap();
    //
    //     let main_result = class_loader
    //         .load_init_class(String::from("mypack/SimpleMain"))
    //         .and_then(|klass| invoke_main(&class_loader, &klass));
    //
    //     main_result.expect("Main exited with non-zero error code");
    //     println!("ABC");
    // }
    //
    // fn invoke_main(class_loader: &ClassLoader, klass: &Klass) -> Result<JvmValue, JvmException> {
    //     let method_by_name = klass.get_method_by_qualified_name(Qualifier::MethodRef {
    //         class_name: String::from("mypack/SimpleMain"),
    //         descriptor: String::from("([Ljava/lang/String;)V"),
    //         name: String::from("main"),
    //     });
    //
    //     method_by_name.map_or(Err(JvmException::new()), |method| {
    //         let frame = StackFrame::new(&class_loader, &klass);
    //         frame.execute_method(method, &klass)
    //     })
    // }

    #[test]
    pub fn testBootStrapLoader() {
        log4rs::init_file(
            "/home/barnab/CLionProjects/rust-jvm/log4rs.yml",
            Default::default(),
        )
        .unwrap();

        let locator = ResourceLocator::new(String::from(
            "/home/barnab/CLionProjects/rust-jvm/resources",
        ));
        let heap = Rc::new(JvmHeap::new());
        let context = Rc::new(GlobalContext::new(heap));
        let loader = Rc::new(BootstrapClassLoader::new(locator, context.clone()));
        context.set_class_loader(loader.clone());

        let result = loader
            .load_and_init_class(String::from("java/lang/Object"))
            .expect("Should be able to load Object!");
        println!("ABC");
    }
}
