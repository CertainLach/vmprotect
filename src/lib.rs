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

#[inline(never)]
#[doc(hidden)]
pub fn blackbox() {}
#[inline(never)]
#[doc(hidden)]
pub fn blackbox_value<T>(_value: &mut T) {
    std::hint::black_box(_value);
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

    (cstr $x: literal) => {
        // Safety: interior nuls are checked by `real_c_string` crate
        unsafe { $crate::strings::encrypted_a::EncryptedStringA::new($crate::marker_name!($x)) }
    };
    (cwstr $x: literal) => {{
        unsafe { $crate::strings::encrypted_w::EncryptedStringW::new($crate::marker_name!($x)) }
    }};
}
