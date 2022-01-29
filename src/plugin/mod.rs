mod api_version;
mod init_plugin;
mod plugin_info;

pub use plugin_info::PluginInfo;
pub use init_plugin::InitializedPlugin;
pub use api_version::APIVersion;

pub const INITIALIZE_FN_NAME: &str = "initialize";

extern "C" {
    fn initialize(thing: *mut PluginInfo);
}