use bitflags::bitflags;
use chrono::offset::TimeZone;
use chrono::{Date, Utc};
use enum_primitive::FromPrimitive;
use enum_primitive::{enum_from_primitive, enum_from_primitive_impl, enum_from_primitive_impl_ty};
use std::os::raw::{c_char, c_void};
use std::time::Duration;

/// Original API implementation
#[cfg_attr(all(not(target_os = "macos"), target_pointer_width = "64"), link(name = "VMProtectSDK64"))]
#[cfg_attr(all(target_os = "macos", target_pointer_width = "64"), link(name = "VMProtectSDK"))]
#[cfg_attr(target_pointer_width = "32", link(name = "VMProtectSDK32"))]
extern "system" {
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
    // Activation
    pub fn VMProtectActivateLicense(code: *const c_char, serial: *mut c_char, size: u32) -> u32;
    pub fn VMProtectDeactivateLicense(serial: *const c_char) -> u32;
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
}

bitflags! {
    pub struct VMProtectSerialStateFlags: u32 {
        const CORRUPTED = 0x00000001;
        const INVALID = 0x00000002;
        const BLACKLISTED = 0x00000004;
        const DATE_EXPIRED = 0x00000008;
        const RUNNING_TIME_OVER = 0x00000010;
        const BAD_HWID = 0x00000020;
        const MAX_BUILD_EXPIRED = 0x00000040;
    }
}
impl VMProtectSerialStateFlags {
    pub fn new(value: u32) -> Self {
        VMProtectSerialStateFlags { bits: value }
    }
    #[inline(always)]
    pub fn is_success(&self) -> bool {
        self.is_empty()
    }
    #[inline(always)]
    pub fn is_corrupted(&self) -> bool {
        self.contains(VMProtectSerialStateFlags::CORRUPTED)
    }
    #[inline(always)]
    pub fn is_blacklisted(&self) -> bool {
        self.contains(VMProtectSerialStateFlags::BLACKLISTED)
    }
    #[inline(always)]
    pub fn is_date_expired(&self) -> bool {
        self.contains(VMProtectSerialStateFlags::DATE_EXPIRED)
    }
    #[inline(always)]
    pub fn is_running_time_over(&self) -> bool {
        self.contains(VMProtectSerialStateFlags::RUNNING_TIME_OVER)
    }
    #[inline(always)]
    pub fn is_bad_hwid(&self) -> bool {
        self.contains(VMProtectSerialStateFlags::BAD_HWID)
    }
    #[inline(always)]
    pub fn is_build_expired(&self) -> bool {
        self.contains(VMProtectSerialStateFlags::MAX_BUILD_EXPIRED)
    }
}

#[test]
fn test_serial_state_flags_parsing() {
    assert!(VMProtectSerialStateFlags::empty().is_empty());
    assert_eq!(
        (VMProtectSerialStateFlags::CORRUPTED | VMProtectSerialStateFlags::BAD_HWID).is_success(),
        false
    );
    assert_eq!((VMProtectSerialStateFlags::CORRUPTED).is_corrupted(), true);
}

#[derive(Default, Clone, Copy)]
#[repr(C, packed)]
pub struct VMProtectDate {
    w_year: u16,
    b_month: u8,
    b_day: u8,
}

impl VMProtectDate {
    fn to_user(self) -> Option<Date<Utc>> {
        if self.w_year == 0 && self.b_month == 0 && self.b_day == 0 {
            None
        } else {
            Some(Utc.ymd(self.w_year as i32, self.b_month as u32, self.b_day as u32))
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct VMProtectSerialNumberData {
    state: u32,               // VMProtectSerialStateFlags
    user_name: [u16; 256],    // user name
    email: [u16; 256],        // email
    expire: VMProtectDate,    // date of serial number expiration
    max_build: VMProtectDate, // max date of build, that will accept this key
    running_time: u32,        // running time in minutes
    user_data_length: u8,     // length of user data in bUserData
    user_data: [u8; 255],     // up to 255 bytes of user data
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

impl VMProtectSerialNumberData {
    pub fn to_user(self) -> VMProtectSerialNumberUserData {
        VMProtectSerialNumberUserData {
            state: VMProtectSerialStateFlags::new(self.state),
            user_name: widestring::U16CString::from_vec_with_nul({ self.user_name }.to_vec())
                .unwrap()
                .to_string()
                .unwrap(),
            email: widestring::U16CString::from_vec_with_nul({ self.user_name }.to_vec())
                .unwrap()
                .to_string()
                .unwrap(),
            expire: self.expire.to_user(),
            max_build: self.max_build.to_user(),
            running_time: Duration::from_secs(60 * self.running_time as u64),
            user_data: self.user_data[..self.user_data_length as usize].to_vec(),
        }
    }
}

#[derive(Debug)]
pub struct VMProtectSerialNumberUserData {
    state: VMProtectSerialStateFlags, // VMProtectSerialStateFlags
    user_name: String,                // user name
    email: String,                    // email
    expire: Option<Date<Utc>>,        // date of serial number expiration
    max_build: Option<Date<Utc>>,     // max date of build, that will accept this key
    running_time: Duration,           // running time in minutes
    user_data: Vec<u8>,               // up to 255 bytes of user data
}

enum_from_primitive! {
// Activation
#[repr(u32)]
#[derive(Debug, PartialEq)]
pub enum VMProtectActivationFlags {
    Ok = 0,
    /// Handled by api automatically
    SmallBuffer = 1,
    NoConnection = 2,
    BadReply = 3,
    Banned = 4,
    Corrupted = 5,
    BadCode = 6,
    AlreadyUsed = 7,
    SerialUnknown = 8,
    Expired = 9,
    NotAvailable = 10,
}
}

#[test]
fn test_activation_flags_parsing() {
    assert_eq!(
        VMProtectActivationFlags::from_i32(0),
        Some(VMProtectActivationFlags::Ok)
    );
    assert_eq!(
        VMProtectActivationFlags::from_i32(10),
        Some(VMProtectActivationFlags::NotAvailable)
    );
    assert_eq!(VMProtectActivationFlags::from_i32(11), None);
}
