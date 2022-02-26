use libc::c_char;
use super::ConversationType;

#[repr(C)]
pub struct Conversation {
    pub id: *const c_char,
    pub name: *const c_char,
    pub team_id: *const c_char,
    pub conversation_type: ConversationType
}