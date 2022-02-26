use libc::c_char;

#[repr(C)]
pub struct Team {
    pub id: *const c_char,
    pub name: *const c_char,
}