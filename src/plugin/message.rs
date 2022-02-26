use libc::c_char;

#[repr(C)]
pub struct Message {
    pub body: *const c_char
}
/*
impl InitializedPlugin {
    pub fn set_body(body: String) {
        
    }
}*/