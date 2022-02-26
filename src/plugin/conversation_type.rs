#[repr(C)]
pub enum ConversationType {
    DirectMessage,
    GroupDirectMessage,
    PublicChannel,
    PrivateChannel
}
