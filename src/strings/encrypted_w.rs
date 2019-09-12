use crate::internal;
use std::ffi::OsString;
use std::os::raw::c_void;
use widestring::{U16CStr, U16CString};

pub struct EncryptedStringW<'t>(pub *const i16, pub std::marker::PhantomData<&'t i16>);
impl<'t> Drop for EncryptedStringW<'t> {
    fn drop(&mut self) {
        unsafe { internal::VMProtectFreeString(self.0 as *const c_void) };
    }
}
impl<'t> AsRef<U16CStr> for EncryptedStringW<'t> {
    fn as_ref(&self) -> &U16CStr {
        unsafe { U16CStr::from_ptr_str(self.0 as *const u16) }
    }
}
impl<'t> Into<U16CString> for EncryptedStringW<'t> {
    fn into(self) -> U16CString {
        (self.as_ref(): &U16CStr).to_owned()
    }
}
impl<'t> Into<String> for EncryptedStringW<'t> {
    fn into(self) -> String {
        (self.as_ref(): &U16CStr).to_string().unwrap()
    }
}
impl<'t> Into<OsString> for EncryptedStringW<'t> {
    fn into(self) -> OsString {
        (self.as_ref(): &U16CStr).to_os_string()
    }
}
