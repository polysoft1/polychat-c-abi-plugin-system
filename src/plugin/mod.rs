mod api_version;
mod init_plugin;
mod plugin_info;
mod send_status;
mod message;

pub use plugin_info::PluginInfo;
pub use init_plugin::InitializedPlugin;
pub use api_version::APIVersion;
pub use send_status::SendStatus;
pub use message::Message;

pub const INITIALIZE_FN_NAME: &str = "initialize";

extern "C" {
    pub fn initialize(thing: *mut PluginInfo);
}
