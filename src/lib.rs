use libc::c_void;

type Account = *mut c_void;

extern "C" {
    fn create_account() -> Account;
}
