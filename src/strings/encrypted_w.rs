use std::fmt;
use std::marker::PhantomData;
use std::ops::Deref;
use vmprotect_sys::VMProtectDecryptStringW;
use vmprotect_sys::VMProtectFreeString;
use widestring::U16CStr;

pub struct EncryptedStringW<'t>(*const U16CStr, PhantomData<&'t U16CStr>);
impl EncryptedStringW<'_> {
    /// Do not call this method directly, use macro
    ///
    /// # Safety
    ///
    /// str should be correct c string literal
    #[doc(hidden)]
    #[inline(always)]
    pub unsafe fn new(str: *const i16) -> Self {
        Self(
            unsafe { U16CStr::from_ptr_str(VMProtectDecryptStringW(str).cast()) },
            PhantomData,
        )
    }
}
impl<'t> Drop for EncryptedStringW<'t> {
    fn drop(&mut self) {
        unsafe { VMProtectFreeString(self.0.cast()) };
    }
}
impl Deref for EncryptedStringW<'_> {
    type Target = U16CStr;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        // Safety: pointer is only invalidated on drop
        unsafe { &*self.0 }
    }
}
impl<'t> fmt::Debug for EncryptedStringW<'t> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{:?}", &**self)
    }
}
