use crate::types::*;
use super::api_version::APIVersion;

use std::option::Option;

#[repr(C)]
pub struct PluginInfo {
    pub supported_api: APIVersion,

    pub create_account: Option<extern fn() -> Account>,
    pub destroy_account: Option<extern fn(acc: Account)>,
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
            print: None
        }
    }
}