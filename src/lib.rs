use std::arch::asm;

#[doc(hidden)]
pub use real_c_string::real_c_string as marker_name;
pub use vmprotect_macros::protected as protect;
#[doc(hidden)]
pub use vmprotect_sys;

#[cfg(feature = "licensing")]
pub mod licensing;
#[doc(hidden)]
pub mod markers;
#[cfg(feature = "service")]
pub mod service;
#[cfg(feature = "strings")]
pub mod strings;

#[inline(always)]
#[doc(hidden)]
pub fn blackbox() {
    unsafe {
        asm!("nop");
    }
}

#[inline(always)]
#[doc(hidden)]
pub fn blackbox_value<T>(value: &mut T) {
    let ptr = value as *mut _;
    unsafe {
        asm!("nop {}", in(reg) ptr);
    }
}

#[macro_export]
macro_rules! protected {
    (mutate $x: literal; $code: expr) => {{
        $crate::markers::begin_mutation($crate::marker_name!($x));
        $crate::blackbox();
        let mut ret = $code;
        $crate::blackbox_value(&mut ret);
        $crate::markers::end();
        ret
    }};
    (virtualize $x: literal; $code: expr) => {{
        $crate::markers::begin_virtualization($crate::marker_name!($x));
        $crate::blackbox();
        let mut ret = $code;
        $crate::blackbox_value(&mut ret);
        $crate::markers::end();
        ret
    }};
    (virtualize, lock $x: literal; $code: expr) => {{
        $crate::markers::begin_virtualization_lock_by_key($crate::marker_name!($x));
        $crate::blackbox();
        let mut ret = $code;
        $crate::blackbox_value(&mut ret);
        $crate::markers::end();
        ret
    }};
    (ultra $x: literal; $code: expr) => {{
        $crate::markers::begin_ultra($crate::marker_name!($x));
        $crate::blackbox();
        let mut ret = $code;
        $crate::blackbox_value(&mut ret);
        $crate::markers::end();
        ret
    }};
    (ultra, lock $x: literal; $code: expr) => {{
        $crate::markers::begin_ultra_lock_by_key($crate::marker_name!($x));
        $crate::blackbox();
        let mut ret = $code;
        $crate::blackbox_value(&mut ret);
        $crate::markers::end();
        ret
    }};

    (cstr $x: literal) => {{
        unsafe {$crate::strings::encrypted_a::EncryptedStringA::new(
            $crate::marker_name!($x),
            $x.len(),
        )}
    }};
    (cwstr $x: literal) => {{
        unsafe {$crate::strings::encrypted_a::EncryptedStringW::new(
            $crate::marker_name!($x),
        )}
    }};
    (str $x: literal) => {{
        &$crate::protected!(cstr $x) as &str
    }}
}
