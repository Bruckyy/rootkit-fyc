#![no_std]
extern crate alloc;

#[cfg(not(test))]
extern crate wdk_panic;



mod utils;

use utils::ToUnicodeString;

#[cfg(not(test))]
use wdk_alloc::WdkAllocator;

use wdk::{nt_success, println};
use wdk_sys::{DEVICE_OBJECT, DRIVER_OBJECT, IRP, IRP_MJ_CREATE, NTSTATUS, PCUNICODE_STRING, PDRIVER_DISPATCH, PUNICODE_STRING, STATUS_SUCCESS, UNICODE_STRING,PDEVICE_OBJECT,PIRP};
use wdk_sys::ntddk::{IoCreateDevice,IoDeleteDevice};
use wdk_sys::ntddk::{IoCreateSymbolicLink,IoDeleteSymbolicLink};




#[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: WdkAllocator = WdkAllocator;

#[export_name = "DriverEntry"] // WDF expects a symbol with the name DriverEntry
pub unsafe extern "system" fn driver_entry(
    driver: &mut DRIVER_OBJECT,
    registry_path: PCUNICODE_STRING,
) -> NTSTATUS {
    println!("Hello world!");

    

   let mut device_name: UNICODE_STRING = "\\Device\\MyDevice\0".to_unicode();

   let mut dos_name: UNICODE_STRING = "\\??\\MyDevice\0".to_unicode();

    let mut device:PDEVICE_OBJECT = core::ptr::null_mut();

    let status = IoCreateDevice(driver, 0, &mut device_name, 0, 0, 0, &mut device);

    if !nt_success(status) {
        return status;
    }

    let status = IoCreateSymbolicLink(&mut dos_name, &mut device_name);

    if !nt_success(status) {
        return status;
    }

    (*driver).MajorFunction[IRP_MJ_CREATE as usize] = Some(major_function_create);

    (*driver).DriverUnload = Some(driver_exit);

    STATUS_SUCCESS
}

// Function called when the driver is unloaded (on exit)
extern "C" fn driver_exit(driver: *mut DRIVER_OBJECT) {
    println!("Exiting driver!");
    unsafe { IoDeleteDevice((*driver).DeviceObject);};
    println!("Deleted device!");
    let mut dos_name: UNICODE_STRING = "\\DosDevices\\MyDevice".to_unicode();
    unsafe { IoDeleteSymbolicLink(&mut dos_name);};
}



unsafe extern "C" fn major_function_create(_device: PDEVICE_OBJECT, pirp: PIRP) -> NTSTATUS {
    println!("Major function create called!");
    STATUS_SUCCESS
}




