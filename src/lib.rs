#![feature(type_ascription)]

pub mod internal;
pub mod licensing;
pub mod strings;

/// Check if currently running application is protected by vmprotect
///
/// Dead code will not be elminated
#[inline(always)]
pub fn is_protected() -> bool {
    unsafe { internal::VMProtectIsProtected() == 1 }
}

/// Check presence of debugger
#[inline(always)]
pub fn is_debugger_present(check_kernel_mode: bool) -> bool {
    unsafe { internal::VMProtectIsDebuggerPresent(if check_kernel_mode { 1 } else { 0 }) == 1 }
}

/// Returns true if running inside virtual machine
#[inline(always)]
pub fn is_virtual_machine() -> bool {
    unsafe { internal::VMProtectIsVirtualMachinePresent() == 1 }
}

/// Check if process memory is not damaged/edited
#[inline(always)]
pub fn is_valid_image_crc() -> bool {
    unsafe { internal::VMProtectIsValidImageCRC() == 1 }
}

#[macro_export]
macro_rules! protected {
    ($x: expr; mutate; $code: block) => {{
        let ret;
        unsafe { vmprotect::internal::VMProtectBeginMutation(real_c_string::real_c_string!($x)) };
        ret = $code;
        unsafe {
            vmprotect::internal::VMProtectEnd();
        };
        ret
    }};
    ($x: expr; virtualize false; $code: block) => {{
        let ret;
        unsafe {
            vmprotect::internal::VMProtectBeginVirtualization(real_c_string::real_c_string!($x))
        };
        ret = $code;
        unsafe {
            vmprotect::internal::VMProtectEnd();
        };
        ret
    }};
    ($x: expr; virtualize true; $code: block) => {{
        let ret;
        unsafe {
            vmprotect::internal::VMProtectBeginVirtualizationLockByKey(
                real_c_string::real_c_string!($x),
            )
        };
        ret = $code;
        unsafe {
            vmprotect::internal::VMProtectEnd();
        };
        ret
    }};
    ($x: expr; ultra false; $code: block) => {{
        let ret;
        unsafe { vmprotect::internal::VMProtectBeginUltra(real_c_string::real_c_string!($x)) };
        ret = $code;
        unsafe {
            vmprotect::internal::VMProtectEnd();
        };
        ret
    }};
    ($x: expr; ultra true; $code: block) => {{
        let ret;
        unsafe {
            vmprotect::internal::VMProtectBeginUltraLockByKey(real_c_string::real_c_string!($x))
        };
        ret = $code;
        unsafe {
            vmprotect::internal::VMProtectEnd();
        };
        ret
    }};
    (A; $x: expr) => {
        vmprotect::strings::encrypted_a::EncryptedStringA(unsafe {
            vmprotect::internal::VMProtectDecryptStringA(real_c_string::real_c_string!($x))
        }) as vmprotect::strings::encrypted_a::EncryptedStringA
    };
    (W; $x: expr) => {
        vmprotect::strings::encrypted_w::EncryptedStringW(unsafe {
            vmprotect::internal::VMProtectDecryptStringW(real_c_string::real_c_wstring!($x))
        }) as vmprotect::strings::encrypted_w::EncryptedStringW // To remove mut
    };
}
