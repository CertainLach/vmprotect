use vmprotect_sys::VMProtectIsDebuggerPresent;
use vmprotect_sys::VMProtectIsProtected;
use vmprotect_sys::VMProtectIsValidImageCRC;
use vmprotect_sys::VMProtectIsVirtualMachinePresent;

/// Check if currently running application is protected by vmprotect
///
/// Dead code will not be elminated
#[inline(always)]
pub fn is_protected() -> bool {
    unsafe { VMProtectIsProtected() == 1 }
}

/// Check presence of debugger
#[inline(always)]
pub fn is_debugger_present(check_kernel_mode: bool) -> bool {
    unsafe { VMProtectIsDebuggerPresent(if check_kernel_mode { 1 } else { 0 }) == 1 }
}

/// Returns true if running inside virtual machine
#[inline(always)]
pub fn is_virtual_machine() -> bool {
    unsafe { VMProtectIsVirtualMachinePresent() == 1 }
}

/// Check if process memory is not damaged/edited
#[inline(always)]
pub fn is_valid_image_crc() -> bool {
    unsafe { VMProtectIsValidImageCRC() == 1 }
}
