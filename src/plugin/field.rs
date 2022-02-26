use libc::c_char;
use super::FieldType;

#[repr(C)]
pub struct Field {
    pub name: *const c_char,
    pub field_type: FieldType,
    pub value: *const c_char,
    pub required: bool,
    pub sensitive: bool
}