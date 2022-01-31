#[repr(C)]
pub enum SendStatus {
    Unknown,
    Sent,
    Sending,
    SendingDelayedNetworkError,
    FailedInvalidMessage,
    FailedAuthenticationError,
    FailedPermissionsError,
    FailedRetriesExceeded,
    LocalOnly
}
