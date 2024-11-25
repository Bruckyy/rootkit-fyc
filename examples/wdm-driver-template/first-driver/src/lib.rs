#![no_std]
extern crate alloc;

#[cfg(not(test))]
extern crate wdk_panic;

#[cfg(not(test))]
use wdk_alloc::WdkAllocator;

use wdk::{nt_success, println};
use wdk_sys::{DRIVER_OBJECT, FILE_OPEN, FILE_SHARE_READ, GENERIC_READ, HANDLE, IO_STATUS_BLOCK, NTSTATUS, OBJECT_ATTRIBUTES, PCUNICODE_STRING, STATUS_SUCCESS, FILE_SYNCHRONOUS_IO_NONALERT};
use core::ptr;
#[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: WdkAllocator = WdkAllocator;
use wdk_sys::UNICODE_STRING;
use alloc::vec::Vec;
use core::ptr::null_mut;
use alloc::string::String;

pub type PVOID = *mut core::ffi::c_void;

extern "system" {
    fn PsGetCurrentProcessId() -> *mut PVOID;

    fn PsGetCurrentThreadId() -> *mut PVOID;

    fn MmIsAddressValid(address: PVOID) -> bool;

    fn PsLookupProcessByProcessId(
        ProcessId: *mut PVOID,
        Process: *mut *mut PVOID
    ) -> NTSTATUS;

    fn ZwCreateFile(
        FileHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        AllocationSize: *mut i64,
        FileAttributes: u32,
        ShareAccess: u32,
        CreateDisposition: u32,
        CreateOptions: u32,
        EaBuffer: *mut PVOID,
        EaLength: u32,
    ) -> NTSTATUS;


    fn ZwReadFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: *mut PVOID,
        ApcContext: *mut PVOID,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        Buffer: *mut PVOID,
        Length: u32,
        ByteOffset: *mut i64,
        Key: *mut u32,
    ) -> NTSTATUS;

    fn ZwDeleteFile(
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    fn ZwClose(Handle: HANDLE) -> NTSTATUS;
}




#[export_name = "DriverEntry"] // WDF expects a symbol with the name DriverEntry
pub unsafe extern "system" fn driver_entry(
    driver: &mut DRIVER_OBJECT,
    registry_path: PCUNICODE_STRING,
) -> NTSTATUS {

    // Initialize the driver routines
    (*driver).DriverUnload = Some(driver_exit);
   
    println!("[>] Hello world!");
    println!("[>] Registry path: {:?}", registry_path);

    let value = ptr::read(&driver);
    println!("[>] Value at address: {:?}", value);


    unsafe {
        let current_pid = PsGetCurrentProcessId();
        println!(
            "[>] Current Process ID: {:?}",
            current_pid
        );

        let caller_pid = PsGetCurrentThreadId();
        println!(
            "[>] Current Thread ID: {:?}",
            caller_pid
        );
    }

    let is_valid: bool = unsafe {MmIsAddressValid(0 as PVOID)};
    println!("[>] Is 0x0 address valid: {:?}", is_valid);

    let driver_ptr = driver as *mut DRIVER_OBJECT as PVOID;
    let is_driver_valid = MmIsAddressValid(driver_ptr);
    println!(
        "[>] Is the driver object address ({:p}) valid: {}",
        driver_ptr, is_driver_valid
    );

    unsafe {
        let mut process: *mut PVOID = null_mut();
        let pid = 7724 as *mut PVOID ;
        let status = PsLookupProcessByProcessId(pid, &mut process);
        if nt_success(status) {
            println!("[>] Found process: {:p}", process);
        }
    }


    //Delete file test
    unsafe {
        let mut file_path = create_unicode_string("\\??\\C:\\coucou\\foo1.txt");   
        let mut object_attributes = OBJECT_ATTRIBUTES {
            Length: core::mem::size_of::<OBJECT_ATTRIBUTES>() as u32,
            RootDirectory: null_mut(),
            ObjectName: &mut file_path,
            Attributes: 0x40,
            SecurityDescriptor: null_mut(),
            SecurityQualityOfService: null_mut(),
        };
        let status = ZwDeleteFile(&mut object_attributes);
        if nt_success(status) {
            println!("[>] File successfully deleted!");
        } else {
            println!("[>] Failed to delete file: 0x{:x}", status);
        }
    }

    unsafe {
        let mut file_handle: HANDLE =null_mut();
        let mut io_status_block: IO_STATUS_BLOCK = core::mem::zeroed();
        let mut object_attributes: OBJECT_ATTRIBUTES = core::mem::zeroed();
        
        let mut file_path = create_unicode_string("\\??\\C:\\foo.txt");
    
        let mut object_attributes = OBJECT_ATTRIBUTES {
            Length: core::mem::size_of::<OBJECT_ATTRIBUTES>() as u32,
            RootDirectory: null_mut(),
            ObjectName: &mut file_path,
            Attributes: 0,
            SecurityDescriptor: null_mut(),
            SecurityQualityOfService: null_mut(),
        };
        
        let delete = ZwDeleteFile(null_mut());
        println!("[>] Delete file: {:?}", delete);

        let status = ZwCreateFile(
            &mut file_handle,
            GENERIC_READ,
            &mut object_attributes,
            &mut io_status_block,
            ptr::null_mut(),
            0,
            1,
            3,
            FILE_SYNCHRONOUS_IO_NONALERT,
            ptr::null_mut(),
            0,
        );
    
        if !nt_success(status) {
            println!("[>] [X] NtOpenFile failed: 0x{:x}", status);
        }
        else{
            println!("[>] File successfully opened, handle : {:p}", file_handle);

            let mut buffer: [u8; 512] = [0; 512]; // Taille du buffer
            let mut byte_offset: i64 = 0;
        
            let status = ZwReadFile(
                file_handle,
               null_mut(),
               null_mut(),
               null_mut(),
                &mut io_status_block,
                buffer.as_mut_ptr() as *mut PVOID,
                buffer.len() as u32,
                &mut byte_offset,
               null_mut(),
            );
        
            if !nt_success(status) {
                println!("[>] [x] ZwReadFile failed: 0x{:x}", status);
                ZwClose(file_handle);
            }
            else{

                let mut vec: Vec<char> = Vec::new();
                for char in &buffer {
                    if *char == 0 {
                        break;
                    }
                    vec.push(u8::from(*char) as char);
                }

                let string: String = vec.into_iter().collect();

                println!("[>] Read successful : {:?}", string.as_str());
        
                ZwClose(file_handle);
                println!("[>] File successfully closed!");
            }

        }
        }
    
    STATUS_SUCCESS
}



extern "C" fn driver_exit(driver: *mut DRIVER_OBJECT) {
    println!("[>] [!] Exiting driver!");
}


unsafe fn create_unicode_string(path: &str) -> UNICODE_STRING {
    let mut unicode_string = UNICODE_STRING {
        Length: 0,
        MaximumLength: 0,
        Buffer:null_mut(),
    };

    let utf16: Vec<u16> = path.encode_utf16().chain(Some(0)).collect(); // UTF-16 encode with null termination
    unicode_string.Buffer = utf16.as_ptr() as *mut u16;
    unicode_string.Length = ((utf16.len() - 1) * 2) as u16; // Subtract null terminator for length
    unicode_string.MaximumLength = (utf16.len() * 2) as u16;

    // core::mem::forget(utf16); // Prevent memory deallocation
    unicode_string
}