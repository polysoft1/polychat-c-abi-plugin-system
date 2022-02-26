use crate::types::*;
use super::api_version::APIVersion;
use super::send_status::SendStatus;
use super::message::Message;
use super::auth_method::AuthMethod;
use super::auth_result::AuthResult;
use super::conversation::Conversation;

use std::option::Option;
use std::ptr;

use libc::c_char;

#[repr(C)]
pub struct PluginInfo {
    pub supported_api: APIVersion,
    pub name: *const c_char,
    pub protocol_name: *const c_char,
    pub auth_methods: *const AuthMethod,
    

    pub create_account: Option<extern fn() -> Account>,
    //pub init_account: Option<extern fn()>,
    pub destroy_account: Option<extern fn(acc: Account)>,
    /**
     * Tries to login with the given authmethod, with the given array of strings.
     * 
     * Async, so the final auth result may not be immediately available.
     * TODO: The way to update auth status.
     */
    pub login: Option<extern fn(acc: Account, * const AuthMethod, * const c_char) -> AuthResult>,
    /**
     * Sends a request to retrieve messages for the conversation that the account
     * has access to.
     * The third parameter is the timestamp for which all messages should be older than
     * The fourth parameter is the maximum quantity of messages to retrieve.
     * 
     * Messages should be added using the function TODO.
     */
    pub request_messages: Option<extern fn(acc: Account, conv: Conversation,
        timestamp: u64, limit: u32)>,
    /// Instructs the plugin to post a message in the associated channel.
    /// The lifetime of msg is only guaranteed during the function call.
    /// To keep the message for longer (likely required), make a copy.
    /// TODO: Add way to update future message status as it is done async.
    pub post_message: Option<extern fn(acc: Account, msg: * const Message) -> SendStatus>,
    pub print: Option<extern fn(acc: Account)>,
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
            login: None,
            request_messages: None,
            post_message: None,
            print: None,
            name: ptr::null(),
            protocol_name: ptr::null(),
            auth_methods: ptr::null(),
        }
    }
}
