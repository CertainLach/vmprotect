use std::marker::PhantomData;
use std::os::raw::c_void;
use vmprotect_sys::VMProtectDecryptStringW;
use vmprotect_sys::VMProtectFreeString;
use widestring::U16CStr;

pub struct EncryptedStringW<'t>(*const i16, PhantomData<&'t i16>);
impl EncryptedStringW<'_> {
    /// Do not call this method directly, use macro
    ///
    /// # Safety
    /// 
    /// str should be correct c string literal
    #[doc(hidden)]
    #[inline(always)]
    pub unsafe fn new(str: *const i16) -> Self {
        Self(VMProtectDecryptStringW(str), PhantomData)
    }
}
impl<'t> Drop for EncryptedStringW<'t> {
    fn drop(&mut self) {
        unsafe { VMProtectFreeString(self.0 as *const c_void) };
    }
}
impl From<EncryptedStringW<'_>> for &U16CStr {
    fn from(str: EncryptedStringW<'_>) -> Self {
        unsafe { U16CStr::from_ptr_str(str.0 as *const u16) }
    }
}
