use wdk_sys::LIST_ENTRY;
use core::{str,slice};
use alloc::string::{String, ToString};

pub type PVOID = *mut core::ffi::c_void;

use crate::constants::{ACTIVE_PROCESS_LINKS_OFFSET, PID_OFFSET, TOKEN_OFFSET, IMAGE_FILE_NAME_OFFSET};

#[derive(Clone)]
pub struct TargetProcess {
    pub eprocess: PVOID,
    pub pid: *mut u32,
    pub list_entry: *mut LIST_ENTRY,
    pub token: *mut usize,
    image_file_name_vec: *const [u8; 15],
    pub image_file_name: String,
}

impl TargetProcess {
    pub unsafe fn from_eprocess(eprocess: PVOID) -> Self {
        
        let pid = (eprocess as usize + PID_OFFSET) as *mut u32;
        let list_entry = (eprocess as usize + ACTIVE_PROCESS_LINKS_OFFSET) as *mut LIST_ENTRY;
        let token = (eprocess as usize + TOKEN_OFFSET) as *mut usize;
        let image_file_name_vec =(eprocess as usize + IMAGE_FILE_NAME_OFFSET) as *const [u8; 15];

        let image_file_name = str::from_utf8(slice::from_raw_parts(image_file_name_vec as *const u8, 15)).unwrap().trim_matches(char::from(0)).to_string();

        TargetProcess {
            eprocess,
            pid,
            list_entry,
            token,
            image_file_name_vec,
            image_file_name,
        }
    }

    pub unsafe fn next(&self) -> Self {
        let next_eprocess = ((*self.list_entry).Flink as usize - ACTIVE_PROCESS_LINKS_OFFSET) as PVOID;
        TargetProcess::from_eprocess(next_eprocess)
    }

}