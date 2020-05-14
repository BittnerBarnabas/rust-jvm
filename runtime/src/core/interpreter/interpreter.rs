use crate::core::heap::jvm_object::Oop;
use crate::core::interpreter::evaluation_stack::EvaluationStack;
use crate::core::interpreter::local_variables::JvmLocalVariableStore;
use crate::core::jvm_exception::JvmException;
use crate::core::jvm_value::JvmValue;
use crate::core::klass::constant_pool::Qualifier;
use crate::core::opcode;
use crate::core::stack_frame::JvmStackFrame;

#[cfg(test)]
#[path = "./interpreter_test.rs"]
mod interpreter_test;

pub fn interpret(
    current_frame: &impl JvmStackFrame,
    byte_codes: &Vec<u8>,
    local_variables: &mut impl JvmLocalVariableStore,
) -> Result<JvmValue, JvmException> {
    let mut ip = 0;
    let mut eval_stack = EvaluationStack::new();
    loop {
        match byte_codes.get(ip) {
            Some(byte_code) => match byte_code {
                &opcode::NOP => {}
                &opcode::ACONST_NULL => panic!("UnImplemented byte-code: ACONST_NULL"),
                &opcode::ICONST_M1 => eval_stack.i_constant(-1),
                &opcode::ICONST_0 => eval_stack.i_constant(0),
                &opcode::ICONST_1 => eval_stack.i_constant(1),
                &opcode::ICONST_2 => eval_stack.i_constant(2),
                &opcode::ICONST_3 => eval_stack.i_constant(3),
                &opcode::ICONST_4 => eval_stack.i_constant(4),
                &opcode::ICONST_5 => eval_stack.i_constant(5),
                &opcode::LCONST_0 => panic!("UnImplemented byte-code: LCONST_0"),
                &opcode::LCONST_1 => panic!("UnImplemented byte-code: LCONST_1"),
                &opcode::FCONST_0 => panic!("UnImplemented byte-code: FCONST_0"),
                &opcode::FCONST_1 => panic!("UnImplemented byte-code: FCONST_1"),
                &opcode::FCONST_2 => panic!("UnImplemented byte-code: FCONST_2"),
                &opcode::DCONST_0 => panic!("UnImplemented byte-code: DCONST_0"),
                &opcode::DCONST_1 => panic!("UnImplemented byte-code: DCONST_1"),
                &opcode::BIPUSH => eval_stack.push(JvmValue::Int {
                    val: read_u8(byte_codes, &mut ip) as i32,
                }),
                &opcode::SIPUSH => panic!("UnImplemented byte-code: SIPUSH"),
                &opcode::LDC => panic!("UnImplemented byte-code: LDC"),
                &opcode::LDC_W => panic!("UnImplemented byte-code: LDC_W"),
                &opcode::LDC2_W => panic!("UnImplemented byte-code: LDC2_W"),
                &opcode::ILOAD => {
                    eval_stack.push(local_variables.load(read_u8(byte_codes, &mut ip)))
                }
                &opcode::LLOAD => panic!("UnImplemented byte-code: LLOAD"),
                &opcode::FLOAD => panic!("UnImplemented byte-code: FLOAD"),
                &opcode::DLOAD => panic!("UnImplemented byte-code: DLOAD"),
                &opcode::ALOAD => panic!("UnImplemented byte-code: ALOAD"),
                &opcode::ILOAD_0 => eval_stack.push(local_variables.load(0)),
                &opcode::ILOAD_1 => eval_stack.push(local_variables.load(1)),
                &opcode::ILOAD_2 => eval_stack.push(local_variables.load(2)),
                &opcode::ILOAD_3 => eval_stack.push(local_variables.load(3)),
                &opcode::LLOAD_0 => panic!("UnImplemented byte-code: LLOAD_0"),
                &opcode::LLOAD_1 => panic!("UnImplemented byte-code: LLOAD_1"),
                &opcode::LLOAD_2 => panic!("UnImplemented byte-code: LLOAD_2"),
                &opcode::LLOAD_3 => panic!("UnImplemented byte-code: LLOAD_3"),
                &opcode::FLOAD_0 => panic!("UnImplemented byte-code: FLOAD_0"),
                &opcode::FLOAD_1 => panic!("UnImplemented byte-code: FLOAD_1"),
                &opcode::FLOAD_2 => panic!("UnImplemented byte-code: FLOAD_2"),
                &opcode::FLOAD_3 => panic!("UnImplemented byte-code: FLOAD_3"),
                &opcode::DLOAD_0 => panic!("UnImplemented byte-code: DLOAD_0"),
                &opcode::DLOAD_1 => panic!("UnImplemented byte-code: DLOAD_1"),
                &opcode::DLOAD_2 => panic!("UnImplemented byte-code: DLOAD_2"),
                &opcode::DLOAD_3 => panic!("UnImplemented byte-code: DLOAD_3"),
                &opcode::ALOAD_0 => panic!("UnImplemented byte-code: ALOAD_0"),
                &opcode::ALOAD_1 => panic!("UnImplemented byte-code: ALOAD_1"),
                &opcode::ALOAD_2 => panic!("UnImplemented byte-code: ALOAD_2"),
                &opcode::ALOAD_3 => panic!("UnImplemented byte-code: ALOAD_3"),
                &opcode::IALOAD => panic!("UnImplemented byte-code: IALOAD"),
                &opcode::LALOAD => panic!("UnImplemented byte-code: LALOAD"),
                &opcode::FALOAD => panic!("UnImplemented byte-code: FALOAD"),
                &opcode::DALOAD => panic!("UnImplemented byte-code: DALOAD"),
                &opcode::AALOAD => panic!("UnImplemented byte-code: AALOAD"),
                &opcode::BALOAD => panic!("UnImplemented byte-code: BALOAD"),
                &opcode::CALOAD => panic!("UnImplemented byte-code: CALOAD"),
                &opcode::SALOAD => panic!("UnImplemented byte-code: SALOAD"),
                &opcode::ISTORE => {
                    local_variables.store(eval_stack.pop(), read_u8(byte_codes, &mut ip))
                }
                &opcode::LSTORE => panic!("UnImplemented byte-code: LSTORE"),
                &opcode::FSTORE => panic!("UnImplemented byte-code: FSTORE"),
                &opcode::DSTORE => panic!("UnImplemented byte-code: DSTORE"),
                &opcode::ASTORE => panic!("UnImplemented byte-code: ASTORE"),
                &opcode::ISTORE_0 => local_variables.store(eval_stack.pop(), 0),
                &opcode::ISTORE_1 => local_variables.store(eval_stack.pop(), 1),
                &opcode::ISTORE_2 => local_variables.store(eval_stack.pop(), 2),
                &opcode::ISTORE_3 => local_variables.store(eval_stack.pop(), 3),
                &opcode::LSTORE_0 => panic!("UnImplemented byte-code: LSTORE_0"),
                &opcode::LSTORE_1 => panic!("UnImplemented byte-code: LSTORE_1"),
                &opcode::LSTORE_2 => panic!("UnImplemented byte-code: LSTORE_2"),
                &opcode::LSTORE_3 => panic!("UnImplemented byte-code: LSTORE_3"),
                &opcode::FSTORE_0 => panic!("UnImplemented byte-code: FSTORE_0"),
                &opcode::FSTORE_1 => panic!("UnImplemented byte-code: FSTORE_1"),
                &opcode::FSTORE_2 => panic!("UnImplemented byte-code: FSTORE_2"),
                &opcode::FSTORE_3 => panic!("UnImplemented byte-code: FSTORE_3"),
                &opcode::DSTORE_0 => panic!("UnImplemented byte-code: DSTORE_0"),
                &opcode::DSTORE_1 => panic!("UnImplemented byte-code: DSTORE_1"),
                &opcode::DSTORE_2 => panic!("UnImplemented byte-code: DSTORE_2"),
                &opcode::DSTORE_3 => panic!("UnImplemented byte-code: DSTORE_3"),
                &opcode::ASTORE_0 => panic!("UnImplemented byte-code: ASTORE_0"),
                &opcode::ASTORE_1 => panic!("UnImplemented byte-code: ASTORE_1"),
                &opcode::ASTORE_2 => panic!("UnImplemented byte-code: ASTORE_2"),
                &opcode::ASTORE_3 => panic!("UnImplemented byte-code: ASTORE_3"),
                &opcode::IASTORE => panic!("UnImplemented byte-code: IASTORE"),
                &opcode::LASTORE => panic!("UnImplemented byte-code: LASTORE"),
                &opcode::FASTORE => panic!("UnImplemented byte-code: FASTORE"),
                &opcode::DASTORE => panic!("UnImplemented byte-code: DASTORE"),
                &opcode::AASTORE => panic!("UnImplemented byte-code: AASTORE"),
                &opcode::BASTORE => panic!("UnImplemented byte-code: BASTORE"),
                &opcode::CASTORE => panic!("UnImplemented byte-code: CASTORE"),
                &opcode::SASTORE => panic!("UnImplemented byte-code: SASTORE"),
                &opcode::POP => panic!("UnImplemented byte-code: POP"),
                &opcode::POP2 => panic!("UnImplemented byte-code: POP2"),
                &opcode::DUP => {
                    let val1 = eval_stack.pop();
                    eval_stack.push(val1.clone());
                    eval_stack.push(val1);
                }
                &opcode::DUP_X1 => panic!("UnImplemented byte-code: DUP_X1"),
                &opcode::DUP_X2 => panic!("UnImplemented byte-code: DUP_X2"),
                &opcode::DUP2 => panic!("UnImplemented byte-code: DUP2"),
                &opcode::DUP2_X1 => panic!("UnImplemented byte-code: DUP2_X1"),
                &opcode::DUP2_X2 => panic!("UnImplemented byte-code: DUP2_X2"),
                &opcode::SWAP => panic!("UnImplemented byte-code: SWAP"),
                &opcode::IADD => eval_stack.add(),
                &opcode::LADD => panic!("UnImplemented byte-code: LADD"),
                &opcode::FADD => panic!("UnImplemented byte-code: FADD"),
                &opcode::DADD => panic!("UnImplemented byte-code: DADD"),
                &opcode::ISUB => panic!("UnImplemented byte-code: ISUB"),
                &opcode::LSUB => panic!("UnImplemented byte-code: LSUB"),
                &opcode::FSUB => panic!("UnImplemented byte-code: FSUB"),
                &opcode::DSUB => panic!("UnImplemented byte-code: DSUB"),
                &opcode::IMUL => panic!("UnImplemented byte-code: IMUL"),
                &opcode::LMUL => panic!("UnImplemented byte-code: LMUL"),
                &opcode::FMUL => panic!("UnImplemented byte-code: FMUL"),
                &opcode::DMUL => panic!("UnImplemented byte-code: DMUL"),
                &opcode::IDIV => panic!("UnImplemented byte-code: IDIV"),
                &opcode::LDIV => panic!("UnImplemented byte-code: LDIV"),
                &opcode::FDIV => panic!("UnImplemented byte-code: FDIV"),
                &opcode::DDIV => panic!("UnImplemented byte-code: DDIV"),
                &opcode::IREM => panic!("UnImplemented byte-code: IREM"),
                &opcode::LREM => panic!("UnImplemented byte-code: LREM"),
                &opcode::FREM => panic!("UnImplemented byte-code: FREM"),
                &opcode::DREM => panic!("UnImplemented byte-code: DREM"),
                &opcode::INEG => panic!("UnImplemented byte-code: INEG"),
                &opcode::LNEG => panic!("UnImplemented byte-code: LNEG"),
                &opcode::FNEG => panic!("UnImplemented byte-code: FNEG"),
                &opcode::DNEG => panic!("UnImplemented byte-code: DNEG"),
                &opcode::ISHL => panic!("UnImplemented byte-code: ISHL"),
                &opcode::LSHL => panic!("UnImplemented byte-code: LSHL"),
                &opcode::ISHR => panic!("UnImplemented byte-code: ISHR"),
                &opcode::LSHR => panic!("UnImplemented byte-code: LSHR"),
                &opcode::IUSHR => panic!("UnImplemented byte-code: IUSHR"),
                &opcode::LUSHR => panic!("UnImplemented byte-code: LUSHR"),
                &opcode::IAND => panic!("UnImplemented byte-code: IAND"),
                &opcode::LAND => panic!("UnImplemented byte-code: LAND"),
                &opcode::IOR => panic!("UnImplemented byte-code: IOR"),
                &opcode::LOR => panic!("UnImplemented byte-code: LOR"),
                &opcode::IXOR => panic!("UnImplemented byte-code: IXOR"),
                &opcode::LXOR => panic!("UnImplemented byte-code: LXOR"),
                &opcode::IINC => panic!("UnImplemented byte-code: IINC"),
                &opcode::I2L => panic!("UnImplemented byte-code: I2L"),
                &opcode::I2F => panic!("UnImplemented byte-code: I2F"),
                &opcode::I2D => panic!("UnImplemented byte-code: I2D"),
                &opcode::L2I => panic!("UnImplemented byte-code: L2I"),
                &opcode::L2F => panic!("UnImplemented byte-code: L2F"),
                &opcode::L2D => panic!("UnImplemented byte-code: L2D"),
                &opcode::F2I => panic!("UnImplemented byte-code: F2I"),
                &opcode::F2L => panic!("UnImplemented byte-code: F2L"),
                &opcode::F2D => panic!("UnImplemented byte-code: F2D"),
                &opcode::D2I => panic!("UnImplemented byte-code: D2I"),
                &opcode::D2L => panic!("UnImplemented byte-code: D2L"),
                &opcode::D2F => panic!("UnImplemented byte-code: D2F"),
                &opcode::I2B => panic!("UnImplemented byte-code: I2B"),
                &opcode::I2C => panic!("UnImplemented byte-code: I2C"),
                &opcode::I2S => panic!("UnImplemented byte-code: I2S"),
                &opcode::LCMP => panic!("UnImplemented byte-code: LCMP"),
                &opcode::FCMPL => panic!("UnImplemented byte-code: FCMPL"),
                &opcode::FCMPG => panic!("UnImplemented byte-code: FCMPG"),
                &opcode::DCMPL => panic!("UnImplemented byte-code: DCMPL"),
                &opcode::DCMPG => panic!("UnImplemented byte-code: DCMPG"),
                &opcode::IFEQ => panic!("UnImplemented byte-code: IFEQ"),
                &opcode::IFNE => panic!("UnImplemented byte-code: IFNE"),
                &opcode::IFLT => panic!("UnImplemented byte-code: IFLT"),
                &opcode::IFGE => panic!("UnImplemented byte-code: IFGE"),
                &opcode::IFGT => panic!("UnImplemented byte-code: IFGT"),
                &opcode::IFLE => panic!("UnImplemented byte-code: IFLE"),
                &opcode::IF_ICMPEQ => panic!("UnImplemented byte-code: IF_ICMPEQ"),
                &opcode::IF_ICMPNE => panic!("UnImplemented byte-code: IF_ICMPNE"),
                &opcode::IF_ICMPLT => panic!("UnImplemented byte-code: IF_ICMPLT"),
                &opcode::IF_ICMPGE => panic!("UnImplemented byte-code: IF_ICMPGE"),
                &opcode::IF_ICMPGT => panic!("UnImplemented byte-code: IF_ICMPGT"),
                &opcode::IF_ICMPLE => panic!("UnImplemented byte-code: IF_ICMPLE"),
                &opcode::IF_ACMPEQ => panic!("UnImplemented byte-code: IF_ACMPEQ"),
                &opcode::IF_ACMPNE => panic!("UnImplemented byte-code: IF_ACMPNE"),
                &opcode::GOTO => panic!("UnImplemented byte-code: GOTO"),
                &opcode::JSR => panic!("UnImplemented byte-code: JSR"),
                &opcode::RET => panic!("UnImplemented byte-code: RET"),
                &opcode::TABLESWITCH => panic!("UnImplemented byte-code: TABLESWITCH"),
                &opcode::LOOKUPSWITCH => panic!("UnImplemented byte-code: LOOKUPSWITCH"),
                &opcode::IRETURN => {
                    return match eval_stack.pop() {
                        java_int @ JvmValue::Int { val: _ } => Ok(java_int),
                        _ => Err(JvmException::from_str(
                            "Non-int value was found on top of stack when executing IRETURN",
                        )),
                    };
                }
                &opcode::LRETURN => panic!("UnImplemented byte-code: LRETURN"),
                &opcode::FRETURN => panic!("UnImplemented byte-code: FRETURN"),
                &opcode::DRETURN => panic!("UnImplemented byte-code: DRETURN"),
                &opcode::ARETURN => panic!("UnImplemented byte-code: ARETURN"),
                &opcode::RETURN => return Ok(JvmValue::Void {}),
                &opcode::GETSTATIC => panic!("UnImplemented byte-code: GETSTATIC"),
                &opcode::PUTSTATIC => panic!("UnImplemented byte-code: PUTSTATIC"),
                &opcode::GETFIELD => panic!("UnImplemented byte-code: GETFIELD"),
                &opcode::PUTFIELD => panic!("UnImplemented byte-code: PUTFIELD"),
                &opcode::INVOKEVIRTUAL => panic!("UnImplemented byte-code: INVOKEVIRTUAL"),
                &opcode::INVOKESPECIAL => panic!("UnImplemented byte-code: INVOKESPECIAL"),
                &opcode::INVOKESTATIC => {
                    let index = read_u16(byte_codes, &mut ip);

                    let qualified_method_name = current_frame
                        .current_class()
                        .constant_pool()
                        .get_qualified_name(index);

                    let method_to_call = current_frame
                        .class_loader()
                        .lookup_static_method(qualified_method_name)?;

                    eval_stack.push(
                        current_frame
                            .execute_method(method_to_call, current_frame.current_class())?,
                    );
                }
                &opcode::INVOKEINTERFACE => panic!("UnImplemented byte-code: INVOKEINTERFACE"),
                &opcode::INVOKEDYNAMIC => panic!("UnImplemented byte-code: INVOKEDYNAMIC"),
                &opcode::NEW => {
                    let index = read_u16(byte_codes, &mut ip);

                    let qualified_klass_name = match current_frame
                        .current_class()
                        .constant_pool()
                        .get_qualified_name(index)
                    {
                        Qualifier::Class { name } => name,
                        _ => return Err(JvmException::new()),
                    };

                    let klass = current_frame
                        .class_loader()
                        .find_or_load_class(qualified_klass_name)?;
                    let new_obj = Oop::build_default_object(klass);

                    let obj_ref = current_frame.class_loader().get_heap().store(new_obj)?;

                    eval_stack.push(obj_ref);
                }
                &opcode::NEWARRAY => panic!("UnImplemented byte-code: NEWARRAY"),
                &opcode::ANEWARRAY => panic!("UnImplemented byte-code: ANEWARRAY"),
                &opcode::ARRAYLENGTH => panic!("UnImplemented byte-code: ARRAYLENGTH"),
                &opcode::ATHROW => panic!("UnImplemented byte-code: ATHROW"),
                &opcode::CHECKCAST => panic!("UnImplemented byte-code: CHECKCAST"),
                &opcode::INSTANCEOF => panic!("UnImplemented byte-code: INSTANCEOF"),
                &opcode::MONITORENTER => panic!("UnImplemented byte-code: MONITORENTER"),
                &opcode::MONITOREXIT => panic!("UnImplemented byte-code: MONITOREXIT"),
                &opcode::WIDE => panic!("UnImplemented byte-code: WIDE"),
                &opcode::MULTIANEWARRAY => panic!("UnImplemented byte-code: MULTIANEWARRAY"),
                &opcode::IFNULL => panic!("UnImplemented byte-code: IFNULL"),
                &opcode::IFNONNULL => panic!("UnImplemented byte-code: IFNONNULL"),
                &opcode::GOTO_W => panic!("UnImplemented byte-code: GOTO_W"),
                &opcode::JSR_W => panic!("UnImplemented byte-code: JSR_W"),
                &opcode::BREAKPOINT => panic!("UnImplemented byte-code: BREAKPOINT"),
                &opcode::IMPDEP1 => panic!("UnImplemented byte-code: IMPDEP1"),
                &opcode::IMPDEP2 => panic!("UnImplemented byte-code: IMPDEP2"),
                _ => panic!("Impossible to reach..."),
            },
            None => {
                panic!("Malformed array of byte codes! Should have been terminated with Return")
            }
        }
        ip += 1;
    }
}

fn read_u8(byte_codes: &Vec<u8>, ip: &mut usize) -> u8 {
    *ip += 1;
    byte_codes[*ip]
}

fn read_u16(byte_codes: &Vec<u8>, ip: &mut usize) -> u16 {
    *ip += 2;
    ((byte_codes[*ip - 1] as u16) << 8) + byte_codes[*ip] as u16
}
