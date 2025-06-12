use std::ffi::CStr;
use std::fmt;
use std::marker::PhantomData;
use std::ops::Deref;
use std::os::raw::c_char;
use vmprotect_sys::VMProtectDecryptStringA;
use vmprotect_sys::VMProtectFreeString;

pub struct EncryptedStringA<'t>(*const CStr, PhantomData<&'t CStr>);
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
    pub unsafe fn new(str: *const c_char) -> Self {
        Self(CStr::from_ptr(VMProtectDecryptStringA(str)), PhantomData)
    }
}
impl<'t> Drop for EncryptedStringA<'t> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe { VMProtectFreeString(self.0.cast()) };
    }
}
impl Deref for EncryptedStringA<'_> {
    type Target = CStr;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
impl<'t> fmt::Debug for EncryptedStringA<'t> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{:?}", &**self)
    }
}
