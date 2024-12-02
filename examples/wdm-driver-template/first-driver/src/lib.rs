#![no_std]
extern crate alloc;

#[cfg(not(test))]
extern crate wdk_panic;

#[cfg(not(test))]
use wdk_alloc::WdkAllocator;

use wdk::{nt_success, println};
use wdk_sys::{STATUS_SUCCESS, DRIVER_OBJECT, NTSTATUS, PCUNICODE_STRING};

#[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: WdkAllocator = WdkAllocator;

#[export_name = "DriverEntry"] // WDF expects a symbol with the name DriverEntry
pub unsafe extern "system" fn driver_entry(
    driver: &mut DRIVER_OBJECT,
    registry_path: PCUNICODE_STRING,
) -> NTSTATUS {
    println!("Hello world!");

    // Afficher ici l'adresse de la fonction driver_exit
    println!("Adresse de la fonction driver_exit: {:p}", driver_exit as extern "C" fn(*mut DRIVER_OBJECT));
    (*driver).DriverUnload = Some(driver_exit);
    // Afficher l'objet driver (DRIVER_OBJECT)
    println!("{:?}", driver);

    STATUS_SUCCESS
}

// Function called when the driver is unloaded (on exit)
extern "C" fn driver_exit(driver: *mut DRIVER_OBJECT) {
    println!("Exiting driver!");
}