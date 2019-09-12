use std::os::raw::{c_char, c_void};

use crate::internal;
use std::ffi::{CStr, CString};

pub struct EncryptedStringA<'t>(pub *const c_char, pub std::marker::PhantomData<&'t c_char>);
impl<'t> Drop for EncryptedStringA<'t> {
    fn drop(&mut self) {
        unsafe { internal::VMProtectFreeString(self.0 as *const c_void) };
    }
}
impl<'t> AsRef<CStr> for EncryptedStringA<'t> {
    fn as_ref(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0) }
    }
}
impl<'t> Into<CString> for EncryptedStringA<'t> {
    fn into(self) -> CString {
        (self.as_ref(): &CStr).to_owned()
    }
}
impl<'t> AsRef<str> for EncryptedStringA<'t> {
    fn as_ref(&self) -> &str {
        (self.as_ref(): &CStr).to_str().unwrap()
    }
}
impl<'t> Into<String> for EncryptedStringA<'t> {
    fn into(self) -> String {
        (self.as_ref(): &str).to_owned()
    }
}
impl<'t> std::fmt::Display for EncryptedStringA<'t> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.as_ref(): &str)
    }
}
impl<'t> std::fmt::Debug for EncryptedStringA<'t> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "{:?}", self.as_ref(): &CStr)
    }
}
