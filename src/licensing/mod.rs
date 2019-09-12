use crate::internal;
use num_traits::cast::FromPrimitive;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Gets hwid to display to user
///
/// Always inlined to make sure its harder than just hooking this function
#[inline(always)]
pub fn get_hwid() -> CString {
    let size = unsafe { internal::VMProtectGetCurrentHWID(0 as *mut c_char, 0) };
    let mut buf: Vec<i8> = Vec::with_capacity(size as usize);
    unsafe { internal::VMProtectGetCurrentHWID(buf.as_mut_ptr(), size) };
    unsafe { CStr::from_ptr(buf.as_ptr()).to_owned() }
}

/// Feeds license system with serial number
#[inline(always)]
pub fn set_serial_number(str: &str) -> internal::VMProtectSerialStateFlags {
    internal::VMProtectSerialStateFlags::new(unsafe {
        internal::VMProtectSetSerialNumber(CString::new(str).unwrap().as_ptr())
    })
}

#[inline(always)]
pub fn get_serial_number_state() -> internal::VMProtectSerialStateFlags {
    internal::VMProtectSerialStateFlags::new(unsafe { internal::VMProtectGetSerialNumberState() })
}

#[inline(always)]
pub fn get_serial_number_data() -> Option<internal::VMProtectSerialNumberUserData> {
    let mut data: internal::VMProtectSerialNumberData =
        internal::VMProtectSerialNumberData::default();
    let out = unsafe {
        internal::VMProtectGetSerialNumberData(
            &mut data as *mut internal::VMProtectSerialNumberData,
            std::mem::size_of::<internal::VMProtectSerialNumberData>() as u32,
        )
    };
    if out == 1 {
        Some(data.to_user())
    } else {
        None
    }
}

#[inline(always)]
pub fn activate_license(code: &str) -> Result<String, internal::VMProtectActivationFlags> {
    // Max possible
    let mut out = Vec::with_capacity(4096 / 8 * 3 / 2 + 64);
    let res = unsafe {
        internal::VMProtectActivateLicense(
            CString::new(code).unwrap().as_ptr(),
            out.as_mut_ptr(),
            out.capacity() as u32,
        )
    };
    let res = internal::VMProtectActivationFlags::from_u32(res).unwrap();
    if res == internal::VMProtectActivationFlags::Ok {
        // Vec buffer is passed to CString
        Ok(unsafe { CStr::from_ptr(out.as_ptr()) }
            .to_str()
            .to_owned()
            .unwrap()
            .to_string())
    } else {
        Err(res)
    }
}

#[inline(always)]
pub fn deactivate_license(serial: &str) -> Result<(), internal::VMProtectActivationFlags> {
    // Max possible
    let res =
        unsafe { internal::VMProtectDeactivateLicense(CString::new(serial).unwrap().as_ptr()) };
    let res = internal::VMProtectActivationFlags::from_u32(res).unwrap();
    if res == internal::VMProtectActivationFlags::Ok {
        Ok(())
    } else {
        Err(res)
    }
}

#[allow(unused_must_use)]
#[test]
fn test_serial() {
    // This test only checks if obfuscated code isn't crashing
    get_hwid();
    set_serial_number("Hello!");
    get_serial_number_state();
    get_serial_number_data();
    activate_license("");
    deactivate_license("123");
}
