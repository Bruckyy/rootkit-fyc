use wdk_sys::LIST_ENTRY;

const ACTIVE_PROCESS_LINKS_OFFSET: usize = 0x448;
const PID_OFFSET: usize = 0x440;

pub type PVOID = *mut core::ffi::c_void;

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