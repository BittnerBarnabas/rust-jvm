use crate::core::klass::attribute::AttributeInfo;

pub struct MethodInfo {
    pub access_flags: u16,
    pub name: String,
    pub descriptor: String,
    pub attributes: Vec<AttributeInfo>,
}

impl MethodInfo {
    pub fn get_code(&self) -> Option<&Vec<u8>> {
        let code = self.attributes.iter().find(|att| match att {
            AttributeInfo::Code { .. } => true,
            _ => false,
        });

        return match code {
            Some(AttributeInfo::Code { code, .. }) => Some(code),
            _ => None,
        };
    }
}
