use std::os::raw::{c_char, c_void};

use crate::internal;
use std::ffi::{CStr, CString};

pub struct EncryptedStringA(pub *const c_char);
impl Drop for EncryptedStringA {
    fn drop(&mut self) {
        unsafe { internal::VMProtectFreeString(self.0 as *const c_void) };
    }
}
impl Into<CString> for EncryptedStringA {
    fn into(self) -> CString {
        unsafe { CStr::from_ptr(self.0).to_owned() }
    }
}
impl Into<String> for EncryptedStringA {
    fn into(self) -> String {
        // Using unwrap is totally safe here, as long
        // EncryptedStringA is created from this library
        // using macroses
        unsafe { CStr::from_ptr(self.0).to_str().unwrap().to_owned() }
    }
}
