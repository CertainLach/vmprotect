use std::os::raw::{c_char, c_void};

/// Original API implementation

extern "C" {
	pub fn VMProtectBegin(name: *const c_char);
	pub fn VMProtectBeginVirtualization(name: *const c_char);
	pub fn VMProtectBeginMutation(name: *const c_char);
	pub fn VMProtectBeginUltra(name: *const c_char);
	pub fn VMProtectBeginVirtualizationLockByKey(name: *const c_char);
	pub fn VMProtectBeginUltraLockByKey(name: *const c_char);
	pub fn VMProtectEnd();
	pub fn VMProtectIsProtected() -> u8;
	pub fn VMProtectIsDebuggerPresent(kernel: u8) -> u8;
	pub fn VMProtectIsVirtualMachinePresent() -> u8;
	pub fn VMProtectIsValidImageCRC() -> u8;
	pub fn VMProtectDecryptStringA(value: *const c_char) -> *const c_char;
	pub fn VMProtectDecryptStringW(value: *const i16) -> *const i16;
	pub fn VMProtectFreeString(value: *const c_void) -> u8;
}

pub enum VMProtectSerialStateFlags {
	Success = 0,
	Corrupted = 0x00000001,
	Invalid = 0x00000002,
	Blacklisted = 0x00000004,
	DateExpired = 0x00000008,
	RunningTimeOver = 0x00000010,
	BadHWID = 0x00000020,
	MaxBuildExpired = 0x00000040,
}

#[repr(C, packed)]
pub struct VMProtectDate {
	w_year: u16,
	b_month: u8,
	b_day: u8,
}

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

extern "C" {
	pub fn VMProtectSetSerialNumber(serial: *const c_char) -> u32;
	pub fn VMProtectGetSerialNumberState() -> u32;
	pub fn VMProtectGetSerialNumberData(data: *const VMProtectSerialNumberData, size: u32) -> u8;
	pub fn VMProtectGetCurrentHWID(hwid: *mut c_char, size: u32) -> u32;
}

// activation
pub enum VMProtectActivationFlags {
	Ok = 0,
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

extern "C" {
	pub fn VMProtectActivateLicense(code: *const c_char, serial: *const c_char, size: u32) -> u32;
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
