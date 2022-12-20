use super::{Team};
use crate::types::Account;

use std::ptr;

pub trait CoreInterface {
    // Note: All functions must have self reference to allow
    // this trait to be made into an object.
    /*fn get_teams(&self, _acc: Account) -> *mut Team {
        return ptr::null_mut();
    }*/

    fn test(&self, test_msg: String) {}
}