use bitflags::bitflags;
use chrono::{offset::TimeZone, Date, Utc};
use std::ffi::NulError;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr::null_mut;
use std::time::Duration;
use vmprotect_sys::{
    VMProtectActivateLicense, VMProtectDate, VMProtectDeactivateLicense, VMProtectGetCurrentHWID,
    VMProtectGetSerialNumberData, VMProtectGetSerialNumberState, VMProtectSerialNumberData,
    VMProtectSetSerialNumber,
};

/// Gets hwid to display to user
///
/// Always inlined to make sure its harder than just hooking this function
#[inline(always)]
pub fn get_hwid() -> String {
    let size = unsafe { VMProtectGetCurrentHWID(null_mut(), 0) };
    let mut buf: Vec<u8> = Vec::with_capacity(size as usize);
    unsafe { VMProtectGetCurrentHWID(buf.as_mut_ptr() as *mut c_char, size) };
    // -1 because vmprotect adds \0 at end of string
    unsafe { buf.set_len(size as usize - 1) };
    // VMProtect docs tells returned string
    unsafe { String::from_utf8_unchecked(buf) }
}

/// Feeds license system with serial number
#[inline(always)]
pub fn set_serial_number(serial: impl Into<Vec<u8>>) -> Result<SerialState, NulError> {
    let serial = CString::new(serial)?;
    Ok(SerialState::new(unsafe {
        VMProtectSetSerialNumber(serial.as_ptr())
    }))
}

#[inline(always)]
pub fn get_serial_number_state() -> SerialState {
    SerialState::new(unsafe { VMProtectGetSerialNumberState() })
}

#[inline(always)]
pub fn get_serial_number_data() -> Option<SerialNumberData> {
    let mut data = VMProtectSerialNumberData::default();
    let out = unsafe {
        VMProtectGetSerialNumberData(
            &mut data as *mut _,
            std::mem::size_of::<VMProtectSerialNumberData>() as u32,
        )
    };
    if out == 1 {
        Some(data.into())
    } else {
        None
    }
}

#[inline(always)]
pub fn activate_license(code: impl Into<Vec<u8>>) -> Result<String, ActivationStatus> {
    let code = CString::new(code).map_err(|_| ActivationStatus::NulError)?;
    // Max possible, from vmprotect examples
    let mut out = Vec::with_capacity(4096 / 8 * 3 / 2 + 64);
    let res =
        unsafe { VMProtectActivateLicense(code.as_ptr(), out.as_mut_ptr(), out.capacity() as u32) };
    let res = ActivationStatus::from(res);
    if res == ActivationStatus::Ok {
        // Vec buffer is passed to CString
        Ok(unsafe { CStr::from_ptr(out.as_ptr()) }
            .to_str()
            .to_owned()
            .unwrap()
            .to_string())
    } else {
        Err(res)
    }
}

#[inline(always)]
pub fn deactivate_license(serial: impl Into<Vec<u8>>) -> Result<(), ActivationStatus> {
    let serial = CString::new(serial).map_err(|_| ActivationStatus::NulError)?;
    // Max possible
    let res = unsafe { VMProtectDeactivateLicense(serial.as_ptr()) };
    let res = ActivationStatus::from(res);
    if res == ActivationStatus::Ok {
        Ok(())
    } else {
        Err(res)
    }
}

bitflags! {
    pub struct SerialState: u32 {
        const CORRUPTED = 0x00000001;
        const INVALID = 0x00000002;
        const BLACKLISTED = 0x00000004;
        const DATE_EXPIRED = 0x00000008;
        const RUNNING_TIME_OVER = 0x00000010;
        const BAD_HWID = 0x00000020;
        const MAX_BUILD_EXPIRED = 0x00000040;
    }
}
impl SerialState {
    pub fn new(value: u32) -> Self {
        SerialState { bits: value }
    }
    #[inline(always)]
    pub fn is_success(&self) -> bool {
        self.is_empty()
    }
    #[inline(always)]
    pub fn is_corrupted(&self) -> bool {
        self.contains(SerialState::CORRUPTED)
    }
    #[inline(always)]
    pub fn is_blacklisted(&self) -> bool {
        self.contains(SerialState::BLACKLISTED)
    }
    #[inline(always)]
    pub fn is_date_expired(&self) -> bool {
        self.contains(SerialState::DATE_EXPIRED)
    }
    #[inline(always)]
    pub fn is_running_time_over(&self) -> bool {
        self.contains(SerialState::RUNNING_TIME_OVER)
    }
    #[inline(always)]
    pub fn is_bad_hwid(&self) -> bool {
        self.contains(SerialState::BAD_HWID)
    }
    #[inline(always)]
    pub fn is_build_expired(&self) -> bool {
        self.contains(SerialState::MAX_BUILD_EXPIRED)
    }
}

fn convert_date(date: &VMProtectDate) -> Option<Date<Utc>> {
    if date.w_year == 0 && date.b_month == 0 && date.b_day == 0 {
        return None;
    }
    Some(Utc.ymd(date.w_year as i32, date.b_month as u32, date.b_day as u32))
}

impl From<VMProtectSerialNumberData> for SerialNumberData {
    // Unaligned read performed here
    #[allow(unused_unsafe)]
    fn from(data: VMProtectSerialNumberData) -> Self {
        Self {
            state: SerialState::new(data.state),
            user_name: widestring::U16CString::from_vec_with_nul(
                unsafe { data.user_name }.to_vec(),
            )
            .unwrap()
            .to_string()
            .unwrap(),
            email: widestring::U16CString::from_vec_with_nul(unsafe { data.email }.to_vec())
                .unwrap()
                .to_string()
                .unwrap(),
            expire: convert_date(&data.expire),
            max_build: convert_date(&data.max_build),
            running_time: Duration::from_secs(60 * data.running_time as u64),
            user_data: data.user_data[0..data.user_data_length as usize].to_vec(),
        }
    }
}

#[derive(Debug)]
pub struct SerialNumberData {
    state: SerialState,
    /// User name
    user_name: String,
    /// Email
    email: String,
    /// Date of serial number expiration
    expire: Option<Date<Utc>>,
    /// Max date of build, that will accept this key
    max_build: Option<Date<Utc>>,
    running_time: Duration,
    user_data: Vec<u8>,
}
impl SerialNumberData {
    #[inline(always)]
    pub fn state(&self) -> SerialState {
        self.state
    }
    #[inline(always)]
    pub fn user_name(&self) -> &str {
        &self.user_name
    }
    #[inline(always)]
    pub fn email(&self) -> &str {
        &self.email
    }
    #[inline(always)]
    pub fn expire(&self) -> Option<Date<Utc>> {
        self.expire
    }
    #[inline(always)]
    pub fn max_build(&self) -> Option<Date<Utc>> {
        self.max_build
    }
    #[inline(always)]
    pub fn running_time(&self) -> Duration {
        self.running_time
    }
    #[inline(always)]
    pub fn user_data(&self) -> &[u8] {
        &self.user_data
    }
}

#[derive(Debug, PartialEq)]
pub enum ActivationStatus {
    Ok,
    /// Handled by api automatically
    SmallBuffer,
    NoConnection,
    BadReply,
    Banned,
    Corrupted,
    BadCode,
    AlreadyUsed,
    SerialUnknown,
    Expired,
    NotAvailable,
    /// Not part of official api
    NulError,
}
impl ActivationStatus {
    fn from(id: u32) -> Self {
        match id {
            0 => Self::Ok,
            1 => Self::SmallBuffer,
            2 => Self::NoConnection,
            3 => Self::BadReply,
            4 => Self::Banned,
            5 => Self::Corrupted,
            6 => Self::BadCode,
            7 => Self::AlreadyUsed,
            8 => Self::SerialUnknown,
            9 => Self::Expired,
            10 => Self::NotAvailable,
            _ => unreachable!(),
        }
    }
}
