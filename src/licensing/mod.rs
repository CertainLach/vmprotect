use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Gets hwid to display to user
///
/// Always inlined to make sure its harder than just hooking this function
#[inline(always)]
pub fn get_hwid() -> CString {
    let size = unsafe { crate::internal::VMProtectGetCurrentHWID(0 as *mut c_char, 0) };
    let mut buf: Vec<i8> = Vec::with_capacity(size as usize);
    unsafe { crate::internal::VMProtectGetCurrentHWID(buf.as_mut_ptr(), size) };
    unsafe { CStr::from_ptr(buf.as_ptr()).to_owned() }
}
