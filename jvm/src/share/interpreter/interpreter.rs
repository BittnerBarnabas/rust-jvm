use crate::share::classfile::constant_pool::{CpInfo, Qualifier};
use crate::share::interpreter::evaluation_stack::EvaluationStack;
use crate::share::interpreter::local_variables::JvmLocalVariableStore;
use crate::share::interpreter::opcode;
use crate::share::runtime::stack_frame::JvmStackFrame;
use crate::share::utilities::jvm_exception::JvmException;
use crate::share::utilities::jvm_value::{JvmValue, PrimitiveType};
use crate::share::utilities::jvm_value::JvmValue::ObjRef;
use crate::share::utilities::jvm_value::ObjectRef::Ref;
use crate::share::native::native_helper_classes::java_lang_String;
use crate::share::utilities::global_symbols::Symbols;
use std::ops::Deref;
use crate::share::memory::oop::Oop;


#[cfg(test)]
#[path = "interpreter_test.rs"]
mod interpreter_test;

mod comparators {
    pub const EQ: fn(i32, i32) -> bool = |lhs, rhs| lhs == rhs;
    pub const NEQ: fn(i32, i32) -> bool = |lhs, rhs| lhs != rhs;
    pub const LE: fn(i32, i32) -> bool = |lhs, rhs| lhs <= rhs;
    pub const LT: fn(i32, i32) -> bool = |lhs, rhs| lhs < rhs;
    pub const GE: fn(i32, i32) -> bool = |lhs, rhs| lhs >= rhs;
    pub const GT: fn(i32, i32) -> bool = |lhs, rhs| lhs > rhs;
}

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
                &opcode::ACONST_NULL => eval_stack.push(JvmValue::null_obj()),
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
                &opcode::LDC => {
                    let index = read_u8(byte_codes, &mut ip);
                    let referenced_cp_entry = current_frame.constant_pool().get(index as usize);

                    match referenced_cp_entry {
                        CpInfo::Integer { bytes } => eval_stack.push(JvmValue::Int { val: bytes.clone() as i32 }),
                        CpInfo::Float { bytes } => eval_stack.push(JvmValue::Float { val: bytes.clone() as f32 }),
                        CpInfo::String { string_index } => {
                            let string_klass = current_frame.class_loader()
                                .load_and_init_class(&Symbols::java_lang_String)?;

                            let string_ref = current_frame.heap()
                                .allocate_object(string_klass)?;

                            let string_contents = current_frame.constant_pool().get_utf8(string_index.clone() as usize)
                                .map(|str| str.into_bytes())
                                .expect("No String reference was found!");

                            let buffer = current_frame.heap().allocate_primitive_array(PrimitiveType::Byte, string_contents.len() as i32)?;
                            buffer.copy_bytes(current_frame.heap().deref(), string_contents);
                            java_lang_String::put_buffer(string_ref.clone(), buffer)?;
                            eval_stack.push(JvmValue::from(string_ref));
                        }
                        _ => {
                            let qualifier = current_frame.constant_pool().get_qualified_name(index as u16);

                            let klass = current_frame
                                .class_loader()
                                .load_class(&qualifier)?;

                            eval_stack.push(JvmValue::from(klass.get_java_mirror()));
                        }
                    }
                }
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
                &opcode::ALOAD_0 => eval_stack.push(local_variables.load(0)),
                &opcode::ALOAD_1 => eval_stack.push(local_variables.load(1)),
                &opcode::ALOAD_2 => eval_stack.push(local_variables.load(2)),
                &opcode::ALOAD_3 => eval_stack.push(local_variables.load(3)),
                &opcode::IALOAD => panic!("UnImplemented byte-code: IALOAD"),
                &opcode::LALOAD => panic!("UnImplemented byte-code: LALOAD"),
                &opcode::FALOAD => panic!("UnImplemented byte-code: FALOAD"),
                &opcode::DALOAD => panic!("UnImplemented byte-code: DALOAD"),
                &opcode::AALOAD => {
                    let index = eval_stack.pop_int()?;
                    let array_ref = eval_stack.pop_ref()?;
                    let object_ref = array_ref.dereference()?.instance_data().get_field(index as usize)?;
                    eval_stack.push(object_ref);
                }
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
                &opcode::ASTORE_0 => local_variables.store(eval_stack.pop(), 0),
                &opcode::ASTORE_1 => local_variables.store(eval_stack.pop(), 1),
                &opcode::ASTORE_2 => local_variables.store(eval_stack.pop(), 2),
                &opcode::ASTORE_3 => local_variables.store(eval_stack.pop(), 3),
                &opcode::IASTORE => panic!("UnImplemented byte-code: IASTORE"),
                &opcode::LASTORE => panic!("UnImplemented byte-code: LASTORE"),
                &opcode::FASTORE => panic!("UnImplemented byte-code: FASTORE"),
                &opcode::DASTORE => panic!("UnImplemented byte-code: DASTORE"),
                &opcode::AASTORE => {
                    let value = eval_stack.pop();
                    let index = eval_stack.pop_int()?;
                    if let JvmValue::ObjRef(array_ref) = eval_stack.pop() {
                        //do a lots of checks here
                        array_ref.dereference()?.instance_data().put_field(index as usize, value)?;
                    } else {
                        return Err(JvmException::from("Stack should contain a Reference."));
                    }
                }
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
                &opcode::IMUL => eval_stack.mul(),
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
                &opcode::IFEQ => eval_if(byte_codes, &mut ip, &mut eval_stack, comparators::EQ)?,
                &opcode::IFNE => eval_if(byte_codes, &mut ip, &mut eval_stack, comparators::NEQ)?,
                &opcode::IFLT => eval_if(byte_codes, &mut ip, &mut eval_stack, comparators::LT)?,
                &opcode::IFGE => eval_if(byte_codes, &mut ip, &mut eval_stack, comparators::GE)?,
                &opcode::IFGT => eval_if(byte_codes, &mut ip, &mut eval_stack, comparators::GT)?,
                &opcode::IFLE => eval_if(byte_codes, &mut ip, &mut eval_stack, comparators::LE)?,
                &opcode::IF_ICMPEQ => eval_if_cmp(byte_codes, &mut ip, &mut eval_stack, comparators::EQ)?,
                &opcode::IF_ICMPNE => eval_if_cmp(byte_codes, &mut ip, &mut eval_stack, comparators::NEQ)?,
                &opcode::IF_ICMPLT => eval_if_cmp(byte_codes, &mut ip, &mut eval_stack, comparators::LT)?,
                &opcode::IF_ICMPGE => eval_if_cmp(byte_codes, &mut ip, &mut eval_stack, comparators::GE)?,
                &opcode::IF_ICMPGT => eval_if_cmp(byte_codes, &mut ip, &mut eval_stack, comparators::GT)?,
                &opcode::IF_ICMPLE => eval_if_cmp(byte_codes, &mut ip, &mut eval_stack, comparators::LE)?,
                &opcode::IF_ACMPEQ => panic!("UnImplemented byte-code: IF_ACMPEQ"),
                &opcode::IF_ACMPNE => panic!("UnImplemented byte-code: IF_ACMPNE"),
                &opcode::GOTO => {
                    let tmp_ip = ip;
                    //TODO make this cleaner as we're adding 1 to ip in all iterations so we need to offset by 1 less
                    let offset = read_u16(byte_codes, &mut ip) - 1;
                    ip = tmp_ip + offset as usize;
                }
                &opcode::JSR => panic!("UnImplemented byte-code: JSR"),
                &opcode::RET => panic!("UnImplemented byte-code: RET"),
                &opcode::TABLESWITCH => panic!("UnImplemented byte-code: TABLESWITCH"),
                &opcode::LOOKUPSWITCH => panic!("UnImplemented byte-code: LOOKUPSWITCH"),
                &opcode::IRETURN => {
                    return match eval_stack.pop() {
                        java_int @ JvmValue::Int { val: _ } => Ok(java_int),
                        _ => Err(JvmException::from(
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
                &opcode::PUTSTATIC => {
                    let index = read_u16(byte_codes, &mut ip);
                    let qualified_name = current_frame
                        .constant_pool()
                        .get_qualified_name(index);

                    let value_to_assign = eval_stack.pop();

                    match qualified_name {
                        Qualifier::FieldRef { class_name, name, type_descriptor } => {
                            let klass = current_frame
                                .class_loader()
                                .load_and_init_class(&class_name)?;

                            klass.get_static_field_by_name_and_type(&name, &type_descriptor)
                                .map(|static_field| static_field.set_static_value(value_to_assign))
                                .ok_or(
                                    JvmException::from(format!("Field not found by {:?}",
                                                               Qualifier::FieldRef { class_name, name, type_descriptor }))
                                )?;
                        }
                        invalid => Err(JvmException::from(format!("PutField index should refer to a field not a {:?}", invalid)))?
                    };
                }
                &opcode::GETFIELD => {
                    let index = read_u16(byte_codes, &mut ip);
                    let qualified_name = current_frame
                        .constant_pool()
                        .get_qualified_name(index);

                    let object_to_modify = eval_stack.pop();

                    match qualified_name {
                        Qualifier::FieldRef { class_name, name, type_descriptor } => {
                            let klass = current_frame
                                .class_loader()
                                .load_and_init_class(&class_name)?;

                            let field_value = klass.get_instance_field_offset(&name, &type_descriptor)
                                .map(|field_offset| {
                                    if let JvmValue::ObjRef(object_ref) = object_to_modify {
                                        //do a lots of checks here
                                        Ok(object_ref.dereference()?.instance_data().get_field(field_offset)?)
                                    } else {
                                        Err(JvmException::from(format!("Stack should contain a Reference to an Object, but was {:?}", object_to_modify)))
                                    }
                                })
                                .ok_or(
                                    JvmException::from(format!("Field not found by {:?}",
                                                               Qualifier::FieldRef { class_name, name, type_descriptor }))
                                )??;
                            eval_stack.push(field_value);
                        }
                        invalid => return Err(JvmException::from(format!("GetField index should refer to a field not a {:?}", invalid)))
                    }
                }
                &opcode::PUTFIELD => {
                    let index = read_u16(byte_codes, &mut ip);
                    let qualified_name = current_frame
                        .constant_pool()
                        .get_qualified_name(index);

                    let value_to_assign = eval_stack.pop();
                    let object_to_modify = eval_stack.pop();

                    match qualified_name {
                        Qualifier::FieldRef { class_name, name, type_descriptor } => {
                            let klass = current_frame
                                .class_loader()
                                .load_and_init_class(&class_name)?;
                            klass.get_instance_field_offset(&name, &type_descriptor)
                                .map(|field_offset| {
                                    if let JvmValue::ObjRef(object_ref) = object_to_modify {
                                        //TODO: do a lots of checks here
                                        object_ref.dereference()?.instance_data().put_field(field_offset, value_to_assign)?;
                                        Ok(())
                                    } else {
                                        Err(JvmException::from(format!("Stack should contain a Reference to an Object, but was {:?}", object_to_modify)))
                                    }
                                })
                                .ok_or(
                                    JvmException::from(format!("Field not found by {:?}",
                                                               Qualifier::FieldRef { class_name, name, type_descriptor }))
                                )??;
                        }
                        invalid => return Err(JvmException::from(format!("PutStatic index should refer to a field not a {:?}", invalid)))
                    }
                }
                &opcode::INVOKEVIRTUAL => panic!("UnImplemented byte-code: INVOKEVIRTUAL"),
                &opcode::INVOKESPECIAL => {
                    let index = read_u16(byte_codes, &mut ip);

                    let qualified_method_name = current_frame
                        .constant_pool()
                        .get_qualified_name(index);

                    let method_to_call = current_frame
                        .class_loader()
                        .lookup_instance_method(qualified_method_name)?;

                    let number_of_parameters = method_to_call.number_of_parameters() + 1;

                    let mut args: Vec<JvmValue> = (0..number_of_parameters).map(|_| eval_stack.pop()).collect();
                    args.reverse();

                    let void_method = method_to_call.is_void();
                    let method_return_value = current_frame.execute_method(method_to_call, args)?;

                    if !void_method {
                        eval_stack.push(method_return_value);
                    }
                }
                &opcode::INVOKESTATIC => {
                    let index = read_u16(byte_codes, &mut ip);

                    let qualified_method_name = current_frame
                        .constant_pool()
                        .get_qualified_name(index);

                    let method_to_call = current_frame
                        .class_loader()
                        .lookup_static_method(qualified_method_name)?;

                    let mut args: Vec<JvmValue> = Vec::new();
                    for _ in 0..method_to_call.number_of_parameters() {
                        args.push(eval_stack.pop());
                    }

                    let void_method = method_to_call.is_void();
                    let method_return_value = current_frame.execute_method(method_to_call, args)?;

                    if !void_method {
                        eval_stack.push(method_return_value);
                    }
                }
                &opcode::INVOKEINTERFACE => panic!("UnImplemented byte-code: INVOKEINTERFACE"),
                &opcode::INVOKEDYNAMIC => panic!("UnImplemented byte-code: INVOKEDYNAMIC"),
                &opcode::NEW => {
                    let index = read_u16(byte_codes, &mut ip);

                    let qualified_klass_name = current_frame
                        .constant_pool()
                        .get_qualified_name(index);

                    let klass = current_frame
                        .class_loader()
                        .load_class(&qualified_klass_name)?;

                    let obj_ref = current_frame.heap().allocate_object(klass)?;

                    eval_stack.push(JvmValue::from(obj_ref));
                }
                &opcode::NEWARRAY => panic!("UnImplemented byte-code: NEWARRAY"),
                &opcode::ANEWARRAY => {
                    let array_size = eval_stack.pop_int()?;
                    let array_type_index = read_u16(byte_codes, &mut ip);

                    let qualified_klass_name = current_frame
                        .constant_pool()
                        .get_qualified_name(array_type_index);

                    let klass = current_frame
                        .class_loader()
                        .load_class(&qualified_klass_name)?;

                    let array_ref = current_frame.heap().allocate_array(klass, array_size)?;
                    eval_stack.push(JvmValue::from(array_ref));
                }
                &opcode::ARRAYLENGTH => {
                    if let JvmValue::ObjRef(array_ref) = eval_stack.pop() {
                        //do a lots of checks here
                        let array_length = match array_ref.dereference()? {
                            Oop::ArrayOop(desc) => Ok(desc.size),
                            Oop::PrimitiveArrayOop(desc) => Ok(desc.size),
                            _ => Err(JvmException::from("Expected array reference!"))
                        }?;
                        eval_stack.push(JvmValue::Int {
                            val: array_length,
                        })
                    } else {
                        return Err(JvmException::from("Stack should contain a Reference."));
                    }
                }
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

fn eval_if_cmp(byte_codes: &Vec<u8>,
               mut ip: &mut usize,
               eval_stack: &mut EvaluationStack,
               comparator: fn(i32, i32) -> bool) -> Result<(), JvmException> {
    let lhs = eval_stack.pop_int()?;
    let rhs = eval_stack.pop_int()?;

    evaluate_conditional(byte_codes, &mut ip, comparator, lhs, rhs)
}

fn eval_if(byte_codes: &Vec<u8>,
           mut ip: &mut usize,
           eval_stack: &mut EvaluationStack,
           comparator: fn(i32, i32) -> bool) -> Result<(), JvmException> {
    let value = eval_stack.pop_int()?;
    evaluate_conditional(byte_codes, &mut ip, comparator, value, 0)
}

fn evaluate_conditional(byte_codes: &Vec<u8>,
                        mut ip: &mut usize,
                        comparator: fn(i32, i32) -> bool,
                        lhs: i32, rhs: i32) -> Result<(), JvmException> {
    let tmp_ip = *ip;

    //TODO make this cleaner as we're adding 1 to ip in all iterations so we need to offset by 1 less
    let offset = read_u16(byte_codes, &mut ip) - 1;

    if comparator(lhs, rhs) {
        *ip = tmp_ip + offset as usize;
    }

    Ok(())
}

fn read_u8(byte_codes: &Vec<u8>, ip: &mut usize) -> u8 {
    *ip += 1;
    byte_codes[*ip]
}

fn read_u16(byte_codes: &Vec<u8>, ip: &mut usize) -> u16 {
    *ip += 2;
    ((byte_codes[*ip - 1] as u16) << 8) + byte_codes[*ip] as u16
}
