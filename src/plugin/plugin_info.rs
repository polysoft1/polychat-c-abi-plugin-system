use crate::types::*;
use super::api_version::APIVersion;
use super::send_status::SendStatus;
use super::message::Message;

use std::option::Option;

#[repr(C)]
pub struct PluginInfo {
    pub supported_api: APIVersion,

    pub create_account: Option<extern fn() -> Account>,
    pub destroy_account: Option<extern fn(acc: Account)>,
    /// Instructs the plugin to post a message in the associated channel.
    /// The lifetime of msg is only guaranteed during the function call.
    /// To keep the message for longer (likely required), make a copy.
    /// TODO: Add way to update future message status as it is done async.
    pub post_message: Option<extern fn(msg: * const Message) -> SendStatus>,
    pub print: Option<extern fn(acc: Account)>
}

impl PluginInfo {
    pub fn new() -> PluginInfo {
        PluginInfo {
            supported_api: APIVersion {
                major: -1,
                minor: 0,
                patch: 0
            },
            create_account: None,
            destroy_account: None,
            post_message: None,
            print: None
        }
    }
}
