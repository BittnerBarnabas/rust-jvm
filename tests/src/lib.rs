#[cfg(test)]
mod tests {
    use runtime::core::class_loader::ClassLoader;
    use runtime::core::class_parser::ClassParser;
    use runtime::core::heap::heap::JvmHeap;
    use runtime::core::jvm_exception::JvmException;
    use runtime::core::jvm_value::JvmValue;
    use runtime::core::klass::constant_pool::Qualifier;
    use runtime::core::klass::klass::Klass;
    use runtime::core::stack_frame::StackFrame;
    use runtime::core::{interpreter, opcode};
    use std::rc::Rc;

    #[test]
    pub fn storing_and_adding_local_integers() {
        let opcodes: Vec<u8> = vec![
            opcode::BIPUSH,
            0x10,
            opcode::ISTORE_1,
            opcode::ICONST_4,
            opcode::ISTORE,
            0x05,
            opcode::ILOAD_1,
            opcode::ILOAD,
            0x05,
            opcode::IADD,
            opcode::ISTORE_3,
            opcode::ILOAD_3,
            opcode::IRETURN,
        ];

        // let frame = StackFrame::new();
        // let interpreter_result = interpreter::interpret(&frame, &opcodes).ok();
        // assert_eq!(interpreter_result, Some(JvmValue::Int { val: 20 }));
    }

    #[test]
    pub fn test() {
        let heap = Rc::new(JvmHeap::new());
        let class_loader = ClassLoader::new(heap.clone());
        class_loader.bootstrap();
        class_loader.load_class(String::from("mypack/SimpleMain"));

        let main_result = class_loader
            .find_or_load_class(String::from("mypack/SimpleMain"))
            .and_then(|klass| invoke_main(&class_loader, &klass));

        println!("ABC");
    }

    fn invoke_main(class_loader: &ClassLoader, klass: &Klass) -> Result<JvmValue, JvmException> {
        let method_by_name = klass.get_method_by_qualified_name(Qualifier::MethodRef {
            class_name: String::from("mypack/SimpleMain"),
            descriptor: String::from("([Ljava/lang/String;)V"),
            name: String::from("main"),
        });

        method_by_name.map_or(Err(JvmException::new()), |method| {
            let frame = StackFrame::new(&class_loader, &klass);
            frame.execute_method(method, &klass)
        })
    }
}
