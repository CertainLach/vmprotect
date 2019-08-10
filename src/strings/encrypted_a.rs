use std::os::raw::{c_char, c_void};

use crate::internal;
use std::borrow::Cow;
use std::ffi::CStr;

pub struct EncryptedStringA(pub *const c_char);
impl Drop for EncryptedStringA {
    fn drop(&mut self) {
        unsafe { internal::VMProtectFreeString(self.0 as *const c_void) };
    }
}
impl Into<&CStr> for EncryptedStringA {
    fn into(self) -> &'static CStr {
        unsafe { CStr::from_ptr(self.0) }
    }
}
impl Into<Cow<'_, str>> for EncryptedStringA {
    fn into(self) -> Cow<'static, str> {
        (self.into(): &CStr).to_string_lossy()
    }
}
impl Into<&str> for EncryptedStringA {
    fn into(self) -> &'static str {
        // Using unwrap is totally safe here, as long
        // EncryptedStringA is created from this library
        // using macroses
        (self.into(): &CStr).to_str().unwrap()
    }
}
impl Into<String> for EncryptedStringA {
    fn into(self) -> String {
        (self.into(): &str).to_string()
    }
}