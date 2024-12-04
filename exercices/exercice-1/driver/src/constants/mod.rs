
use wdk_sys::{FILE_DEVICE_UNKNOWN, METHOD_NEITHER, FILE_ANY_ACCESS};

macro_rules! CTL_CODE {
    ($DeviceType:expr, $Function:expr, $Method:expr, $Access:expr) => {
        ($DeviceType << 16) | ($Access << 14) | ($Function << 2) | $Method
    }
}


pub const IOCTL_PROCESS_HIDE_REQUEST: u32 = CTL_CODE!(FILE_DEVICE_UNKNOWN, 0x805, METHOD_NEITHER, FILE_ANY_ACCESS);