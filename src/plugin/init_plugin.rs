use super::{PluginInfo, APIVersion, Message, SendStatus};
use crate::types::Account;

use libc::c_char;

#[derive(Debug)]
pub struct InitializedPlugin {
    pub supported_api: APIVersion,

    pub create_account: extern fn() -> Account,
    pub destroy_account: extern fn(acc: Account),
    pub post_message: extern fn(msg: * const Message) -> SendStatus,
    pub print: extern fn(acc: Account),
    pub get_name: extern fn() -> *const c_char
}

impl InitializedPlugin {
    pub fn new(plugin: &PluginInfo) -> Result<InitializedPlugin, String> {
        if plugin.create_account.is_none() {
            return Err("create_account is not defined".to_string());
        } else if plugin.destroy_account.is_none() {
            return Err("destroy_account is not defined".to_string());
        } else if plugin.post_message.is_none() {
            return Err("post_message is not defined".to_string());
        } else if plugin.print.is_none() {
            return Err("print is not defined".to_string());
        } else if plugin.get_name.is_none() {
            return Err("get_name is undefined".to_string());
        }

        Ok(InitializedPlugin {
            supported_api: plugin.supported_api,
            create_account: plugin.create_account.unwrap(),
            destroy_account: plugin.destroy_account.unwrap(),
            post_message: plugin.post_message.unwrap(),
            print: plugin.print.unwrap(),
            get_name: plugin.get_name.unwrap(),
        })
    }
}
