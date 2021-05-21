pub mod java_lang_String {
    use crate::share::memory::oop::oops::{ObjectOopDesc, PrimitiveArrayOopDesc};
    use crate::share::utilities::jvm_value::JvmValue;
    use crate::share::memory::heap::{JvmHeap, Heap};
    use crate::share::memory::oop::Oop;
    use crate::share::utilities::jvm_exception::JvmException;
    use std::ops::Deref;

    const BUFFER_OFFSET: usize = 0;

    pub fn put_buffer(string_ref: ObjectOopDesc, buffer: PrimitiveArrayOopDesc) -> Result<(), JvmException> {
        string_ref.instance_data().put_field(BUFFER_OFFSET, JvmValue::from(buffer))
    }

}