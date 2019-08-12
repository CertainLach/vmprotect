#![feature(type_ascription)]

pub mod internal;
pub mod licensing;
pub mod strings;

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
