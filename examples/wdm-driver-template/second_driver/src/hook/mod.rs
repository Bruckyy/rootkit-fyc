use core::ffi::c_void;
use core::ptr::null_mut;
use wdk::{nt_success, println};
use wdk_sys::STATUS_BUFFER_TOO_SMALL;
use wdk_sys::STATUS_INVALID_BUFFER_SIZE;

use wdk_sys::ntddk::EtwRegister;
use wdk_sys::GUID;
use wdk_sys::NTSTATUS;
use wdk_sys::{ULONG, PVOID};
use wdk_sys::ntddk::KeQueryPerformanceCounter;

use wdk_sys::ntddk::MmGetSystemRoutineAddress;
use crate::utils::ToUnicodeString;

const  SystemTraceControlGuid : GUID = GUID {
    Data1: 0x9e814aad,
    Data2: 0x3204,
    Data3: 0x11d2,
    Data4: [0x9a, 0x82, 0x00, 0x60, 0x08, 0xa8, 0x69, 0x39],
};




pub fn hooking_prototype() {

    

    unsafe {
        let FunctionCode = 0x1;
        let InBuffer: *mut c_void = [0;0xbB0].as_mut_ptr() as *mut c_void; 
        let InBufferLen = 0xbB0;
        let OutBuffer: *mut c_void = [0;0xbB0].as_mut_ptr() as *mut c_void;
        let OutBufferLen = 0xbB0;
        let ReturnLength: [u32; 1] = [0];

        let status = ZwTraceControl(FunctionCode, InBuffer , InBufferLen, OutBuffer, OutBufferLen, ReturnLength.as_ptr() as *mut ULONG);
        println!("ZwTraceControl: {:?}", status);
        println!("ReturnLength: {:?}", ReturnLength);
        if status == STATUS_BUFFER_TOO_SMALL {
            println!("STATUS_BUFFER_TOO_SMALL");
        }
        else if status ==  STATUS_INVALID_BUFFER_SIZE {
            println!("STATUS_INVALID_BUFFER_SIZE");
            
        }
        else {
            println!("STATUS: is not STATUS_BUFFER_TOO_SMALL");
        }
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