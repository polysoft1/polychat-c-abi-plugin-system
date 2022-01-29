#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct APIVersion {
    pub major: i32,
    pub minor: i32,
    pub patch: i32
}