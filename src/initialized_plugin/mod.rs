use crate::plugin_info::{PluginInfo, api_version::APIVersion};
use crate::types::Account;

pub struct InitializedPlugin {
    pub supported_api: APIVersion,

    pub create_account: extern fn() -> Account,
    pub destroy_account: extern fn(acc: Account),   
}

impl InitializedPlugin {
    pub fn new(plugin: &PluginInfo) -> Result<InitializedPlugin, &str> {
        if plugin.create_account.is_none() {
            return Err("create_account is not defined");
        } else if plugin.destroy_account.is_none() {
            return Err("destroy_account is not defined");
        }

        Ok(InitializedPlugin {
            supported_api: plugin.supported_api,
            create_account: plugin.create_account.unwrap(),
            destroy_account: plugin.destroy_account.unwrap()
        })
    }
}