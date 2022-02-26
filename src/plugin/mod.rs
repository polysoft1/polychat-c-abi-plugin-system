mod api_version;
mod init_plugin;
mod plugin_info;
mod send_status;
mod field_type;
mod conversation_type;
mod auth_result;
mod auth_method;
mod message;
mod team;
mod conversation;
mod field;
mod polychat_api;
mod core_interface;

pub use plugin_info::PluginInfo;
pub use init_plugin::InitializedPlugin;
pub use api_version::APIVersion;
pub use send_status::SendStatus;
pub use field_type::FieldType;
pub use conversation_type::ConversationType;
pub use auth_result::AuthResult;
pub use team::Team;
pub use conversation::Conversation;
pub use field::Field;
pub use auth_method::AuthMethod;
pub use polychat_api::PolyChatApiV1;
pub use core_interface::CoreInterface;

pub use message::Message;

pub const INITIALIZE_FN_NAME: &str = "initialize";

extern "C" {
    pub fn initialize(plugin_info: *mut PluginInfo, core_api: *const PolyChatApiV1);
}
