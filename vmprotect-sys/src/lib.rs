use std::os::raw::{c_char, c_void};

// Original API implementation
#[cfg_attr(
    all(not(target_os = "macos"), target_pointer_width = "64"),
    link(name = "VMProtectSDK64")
)]
#[cfg_attr(
    all(target_os = "macos", target_pointer_width = "64"),
    link(name = "VMProtectSDK")
)]
#[cfg_attr(target_pointer_width = "32", link(name = "VMProtectSDK32"))]
unsafe extern "system" {
    // Markers
    pub fn VMProtectBegin(name: *const c_char);
    pub fn VMProtectBeginVirtualization(name: *const c_char);
    pub fn VMProtectBeginMutation(name: *const c_char);
    pub fn VMProtectBeginUltra(name: *const c_char);
    pub fn VMProtectBeginVirtualizationLockByKey(name: *const c_char);
    pub fn VMProtectBeginUltraLockByKey(name: *const c_char);
    pub fn VMProtectEnd();
    // Service
    pub fn VMProtectIsProtected() -> u8;
    pub fn VMProtectIsDebuggerPresent(kernel: u8) -> u8;
    pub fn VMProtectIsVirtualMachinePresent() -> u8;
    pub fn VMProtectIsValidImageCRC() -> u8;
    // Also service by vmprotect docs, but here located under strings feature
    pub fn VMProtectDecryptStringA(value: *const c_char) -> *const c_char;
    pub fn VMProtectDecryptStringW(value: *const i16) -> *const i16;
    pub fn VMProtectFreeString(value: *const c_void) -> u8;
    // Licensing
    pub fn VMProtectSetSerialNumber(serial: *const c_char) -> u32;
    pub fn VMProtectGetSerialNumberState() -> u32;
    pub fn VMProtectGetSerialNumberData(data: *mut VMProtectSerialNumberData, size: u32) -> u8;
    pub fn VMProtectGetCurrentHWID(hwid: *mut c_char, size: u32) -> u32;
    pub fn VMProtectGetOfflineActivationString(
        code: *const c_char,
        buf: *const c_char,
        size: u32,
    ) -> u32;
    pub fn VMProtectGetOfflineDeactivationString(
        serial: *const c_char,
        buf: *const c_char,
        size: u32,
    ) -> u32;
    // Activation
    pub fn VMProtectActivateLicense(code: *const c_char, serial: *mut c_char, size: u32) -> u32;
    pub fn VMProtectDeactivateLicense(serial: *const c_char) -> u32;
}

#[derive(Default, Clone, Copy)]
#[repr(C, packed)]
pub struct VMProtectDate {
    pub w_year: u16,
    pub b_month: u8,
    pub b_day: u8,
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct VMProtectSerialNumberData {
    /// State flags
    pub state: u32,
    /// User name
    pub user_name: [u16; 256],
    /// Email
    pub email: [u16; 256],
    /// Date of serial number expiration
    pub expire: VMProtectDate,
    /// Max date of build, that will accept this key
    pub max_build: VMProtectDate,
    /// Running time in minutes
    pub running_time: u32,
    /// Length of user data in bUserData
    pub user_data_length: u8,
    /// Up to 255 bytes of user data
    pub user_data: [u8; 255],
}

impl Default for VMProtectSerialNumberData {
    fn default() -> Self {
        VMProtectSerialNumberData {
            state: 0,
            user_name: [0; 256],
            email: [0; 256],
            expire: VMProtectDate::default(),
            max_build: VMProtectDate::default(),
            running_time: 0,
            user_data_length: 0,
            user_data: [0; 255],
        }
    }
}
