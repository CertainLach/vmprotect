use crate::internal;

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

/// Runs successfully when segfault isn't caused by executing
/// service functions
#[test]
fn sdk_service_functions_isnt_crashing() {
    is_protected();
    is_debugger_present(false);
    is_debugger_present(true);
    is_virtual_machine();
    is_valid_image_crc();
}
