
use super::{Team, CoreInterface};
use crate::types::Account;

use libc::{c_void, c_char};
use std::{
    ffi::CString,
    fmt::Debug,
    fmt::Result,
    fmt::Formatter
};

use log::error;


/**
 * This is the interface that is passed into C that allows the core's functions to be called.
 * It takes in a boxed CoreInterface, allowing the plugin to call items from the trait.
 */
#[repr(C)]
pub struct PolyChatApiV1 {
    core: *mut c_void, // Hidden *mut Box<dyn CoreInterface>,
    // TODO: Eventually add fields identifying info about core
    //get_teams: Option<extern fn(&PolyChatApiV1, acc: Account) -> *mut Team>,
    test: Option<extern fn(&PolyChatApiV1, test_msg: *const c_char)>,
}

impl PolyChatApiV1 {
    pub fn new(core: *mut &Box<dyn CoreInterface>) -> PolyChatApiV1 {
        PolyChatApiV1 {
            core: core as *mut c_void,
            //get_teams: Some(PolyChatApiV1::get_teams_impl)
            test: Some(PolyChatApiV1::test_impl)
        }
    }

    /*extern fn get_teams_impl(&self, acc: Account) -> *mut Team {
        let interface: Box<Box<dyn CoreInterface>> = unsafe { Box::from_raw(self.core as *mut _) };
        return interface.get_teams(acc);
    }*/
    extern fn test_impl(&self, test_msg: *const c_char) {
        let interface: Box<Box<dyn CoreInterface>> = unsafe { Box::from_raw(self.core as *mut _) };
        // Convert to rust string
        unsafe {
            let rust_c_str = CString::from_raw(test_msg as *mut c_char).into_string();
            match rust_c_str {
                Ok(string) => {
                    interface.test(string);
                },
                Err(error) => {
                    // Complain to logs. This should never happen. Incompatible ABI?
                    error!("Error converting from c string to rust string in test_impl {}", error.to_string());
                }
            };
        }
    }
}

impl Debug for PolyChatApiV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_tuple("")
            .finish()
    }
}