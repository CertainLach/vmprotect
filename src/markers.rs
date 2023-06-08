//! Although functions in this module receive raw pointer, it is safe,
//! because VMProtect doesn't do anything with them other than analysing
//! on compile time
//! If name can't be resolved statically - vmprotect will write that
//!
//! Also, this module is only meant to be called by macros

use std::os::raw::c_char;
use vmprotect_sys::VMProtectBeginMutation;
use vmprotect_sys::VMProtectBeginUltra;
use vmprotect_sys::VMProtectBeginUltraLockByKey;
use vmprotect_sys::VMProtectBeginVirtualization;
use vmprotect_sys::VMProtectBeginVirtualizationLockByKey;
use vmprotect_sys::VMProtectEnd;

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[inline(always)]
#[doc(hidden)]
pub fn begin_mutation(str: *const c_char) {
    unsafe { VMProtectBeginMutation(str) };
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[inline(always)]
#[doc(hidden)]
pub fn begin_virtualization(str: *const c_char) {
    unsafe { VMProtectBeginVirtualization(str) };
}
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[inline(always)]
#[doc(hidden)]
pub fn begin_virtualization_lock_by_key(str: *const c_char) {
    unsafe { VMProtectBeginVirtualizationLockByKey(str) };
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[inline(always)]
#[doc(hidden)]
pub fn begin_ultra(str: *const c_char) {
    unsafe { VMProtectBeginUltra(str) };
}
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[inline(always)]
#[doc(hidden)]
pub fn begin_ultra_lock_by_key(str: *const c_char) {
    unsafe { VMProtectBeginUltraLockByKey(str) };
}

#[inline(always)]
#[doc(hidden)]
pub fn end() {
    unsafe { VMProtectEnd() };
}
