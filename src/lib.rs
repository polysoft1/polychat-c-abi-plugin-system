#![allow(dead_code)]
use libc::c_void;

type Account = *mut c_void;

extern "C" {
    fn create_account() -> Account;
    fn print(account: Account);
    fn destroy_account(account: Account);
}
