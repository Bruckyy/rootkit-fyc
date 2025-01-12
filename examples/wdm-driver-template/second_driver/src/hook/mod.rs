use core::ffi::c_void;
use core::ptr::null_mut;
use wdk::{nt_success, println};

use wdk_sys::ntddk::EtwRegister;
use wdk_sys::GUID;
use wdk_sys::NTSTATUS;
use wdk_sys::{ULONG, PVOID};
use wdk_sys::ntddk::KeQueryPerformanceCounter;

use wdk_sys::ntddk::MmGetSystemRoutineAddress;
use crate::utils::ToUnicodeString;


pub fn hooking_prototype() {

    

    unsafe {
        ZwTraceControl(FunctionCode, InBuffer, InBufferLen, OutBuffer, OutBufferLen, ReturnLength);
    }

    



}


pub fn hooking() {
    let mut name = "HalPrivateDispatchTable".to_unicode();
    unsafe {
        let address = MmGetSystemRoutineAddress( &mut name);
        println!("HalPrivateDispatchTable: {:?}", address);
        let HalpCollectPmcCounters = address.wrapping_add(0x248);
        println!("HalpCollectPmcCounters: {:?}", HalpCollectPmcCounters);

        let func_ptr: extern "C" fn() -> () = base_hook;


        let void_ptr = func_ptr as *mut c_void;
        *(HalpCollectPmcCounters as *mut *mut c_void) = void_ptr;
    }
    
}
extern "C"  fn base_hook() {
    println!("base_hook");
}


extern "system" {
    pub fn ZwTraceControl(
        FunctionCode: ULONG,
        InBuffer: PVOID,
        InBufferLen: ULONG,
        OutBuffer: PVOID,
        OutBufferLen: ULONG,
        ReturnLength: *mut ULONG,
    ) -> NTSTATUS;
}