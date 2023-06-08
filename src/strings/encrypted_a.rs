use std::fmt;
use std::marker::PhantomData;
use std::ops::Deref;
use std::os::raw::{c_char, c_void};
use vmprotect_sys::VMProtectDecryptStringA;
use vmprotect_sys::VMProtectFreeString;

pub struct EncryptedStringA<'t>(*const c_char, usize, PhantomData<&'t c_char>);
impl EncryptedStringA<'_> {
    /// Do not call this method directly, use macro
    ///
    /// # Safety
    /// 
    /// str should be correct c string literal
    /// 
    /// len should be length of passed str (in bytes) excluding \0
    #[inline(always)]
    #[doc(hidden)]
    pub unsafe fn new(str: *const c_char, len: usize) -> Self {
        Self(VMProtectDecryptStringA(str), len, PhantomData)
    }
}
impl<'t> Drop for EncryptedStringA<'t> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe { VMProtectFreeString(self.0 as *const c_void) };
    }
}
impl Deref for EncryptedStringA<'_> {
    type Target = str;

    #[inline(always)]
    fn deref(&self) -> &str {
        // Safe, because input is already verified by real_c_string macro
        let slice = unsafe { std::slice::from_raw_parts(self.0 as *const u8, self.1) };
        unsafe { std::str::from_utf8_unchecked(slice) }
    }
}
impl<'t> From<EncryptedStringA<'t>> for String {
    #[inline(always)]
    fn from(val: EncryptedStringA<'t>) -> Self {
        val.to_owned()
    }
}
impl<'t> fmt::Display for EncryptedStringA<'t> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.as_ref() as &str)
    }
}
impl<'t> fmt::Debug for EncryptedStringA<'t> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{:?}", self.as_ref() as &str)
    }
}
