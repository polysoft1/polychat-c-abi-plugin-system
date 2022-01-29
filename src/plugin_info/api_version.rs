#[repr(C)]
#[derive(Copy, Clone)]
pub struct APIVersion {
    pub major: i32,
    pub minor: i32,
    pub patch: i32
}