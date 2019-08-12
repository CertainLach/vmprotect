use crate::internal;
use std::os::raw::c_void;
use widestring::U16CString;

pub struct EncryptedStringW(pub *const i16);
impl Drop for EncryptedStringW {
    fn drop(&mut self) {
        unsafe { internal::VMProtectFreeString(self.0 as *const c_void) };
    }
}
impl Into<U16CString> for EncryptedStringW {
    fn into(self) -> U16CString {
        unsafe { U16CString::from_ptr_str(self.0 as *const u16) }
    }
}
