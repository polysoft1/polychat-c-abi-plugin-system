#[repr(C)]
pub enum AuthResult {
    Success,
    FailRejected,
    FailConnectionError,
    Connecting
}
