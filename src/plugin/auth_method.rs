use libc::c_char;
use super::Field;

#[repr(C)]
pub struct AuthMethod {
    pub name: *const c_char,
    pub num_fields: u32,
    pub fields: *const Field
}