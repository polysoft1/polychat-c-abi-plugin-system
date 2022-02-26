use super::{PluginInfo, APIVersion, Message, SendStatus, AuthMethod,
    AuthResult, Conversation};
use crate::types::Account;

use libc::c_char;
use std::ffi::CString;

#[derive(Debug)]
pub struct InitializedPlugin {
    pub supported_api: APIVersion,
    pub name: String,
    pub protocol_name: String,
    pub auth_methods: *const AuthMethod,

    pub create_account: extern fn() -> Account,
    pub destroy_account: extern fn(acc: Account),
    pub post_message: extern fn(acc: Account, msg: * const Message) -> SendStatus,
    pub login: extern fn(acc: Account, * const AuthMethod, * const c_char) -> AuthResult,
    pub request_messages: extern fn(acc: Account, conv: Conversation,
        timestamp: u64, limit: u32),
    pub print: extern fn(acc: Account),
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
        } else if plugin.name.is_null() {
            return Err("name is not defined".to_string());
        }

        let name: String;
        let protocol_name: String;
        unsafe {
            let name_res = CString::from_raw(plugin.name as *mut c_char).into_string();
            if name_res.is_err() {
                return Err("Could not decode plugin name".to_string());
            }

            name = name_res.unwrap();

            let protocol_name_res = CString::from_raw(plugin.protocol_name as *mut c_char).into_string();
            if protocol_name_res.is_err() {
                return Err("Could not decode plugin protocol name".to_string());
            }

            protocol_name = protocol_name_res.unwrap();
        }

        Ok(InitializedPlugin {
            supported_api: plugin.supported_api,
            create_account: plugin.create_account.unwrap(),
            destroy_account: plugin.destroy_account.unwrap(),
            login: plugin.login.unwrap(),
            request_messages: plugin.request_messages.unwrap(),
            post_message: plugin.post_message.unwrap(),
            print: plugin.print.unwrap(),
            name: name,
            protocol_name: protocol_name,
            auth_methods: plugin.auth_methods
        })
    }
}
