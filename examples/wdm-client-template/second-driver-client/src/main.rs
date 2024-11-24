use std::ffi::c_void;

use windows::Win32::Storage::FileSystem::CreateFileW;
use windows::Win32::Foundation::GENERIC_READ;
use windows::Win32::Foundation::GENERIC_WRITE;
use windows::Win32::Storage::FileSystem::{FILE_GENERIC_READ,FILE_GENERIC_WRITE,FILE_SHARE_READ,FILE_SHARE_WRITE};
use windows::Win32::Storage::FileSystem::{CREATE_ALWAYS,OPEN_EXISTING};
use windows::Win32::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES;
use  windows::Win32::Foundation::CloseHandle;
use windows::core::w;
use windows::Win32::Foundation::HANDLE;

fn main() {
    let filename = w!("\\\\.\\MyDevice");

    unsafe {
    let handle = CreateFileW(filename,
         FILE_GENERIC_READ.0 | FILE_GENERIC_WRITE.0,
          FILE_SHARE_READ | FILE_SHARE_WRITE,
           None,
            OPEN_EXISTING,
            FILE_FLAGS_AND_ATTRIBUTES(0),
              HANDLE(0 as *mut c_void));
            
            

    match handle {
        Ok(handle) => {
            println!("Handle: {:?}", handle);
            CloseHandle(handle).unwrap();
            
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }

    
    }
};
    
    println!("Hello, world!");
}
