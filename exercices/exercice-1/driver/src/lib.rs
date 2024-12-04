#![no_std]
extern crate alloc;

#[cfg(not(test))]
extern crate wdk_panic;

mod utils;
use utils::ToUnicodeString;

mod constants;
use constants::IOCTL_PROCESS_HIDE_REQUEST;


#[cfg(not(test))]
use wdk_alloc::WdkAllocator;

use wdk::{nt_success, println};
use wdk_sys::ntddk::{IoCreateDevice, IoDeleteDevice, IoCreateSymbolicLink, IoDeleteSymbolicLink, IofCompleteRequest};
use wdk_sys::{
    DEVICE_OBJECT, DRIVER_OBJECT, IRP, IRP_MJ_CREATE,IRP_MJ_DEVICE_CONTROL, NTSTATUS, PCUNICODE_STRING, PDEVICE_OBJECT,
    PDRIVER_DISPATCH, PIRP, PUNICODE_STRING, STATUS_SUCCESS, UNICODE_STRING,PIO_STACK_LOCATION, STATUS_UNSUCCESSFUL, IO_NO_INCREMENT
};





#[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: WdkAllocator = WdkAllocator;

#[export_name = "DriverEntry"] // WDF expects a symbol with the name DriverEntry
pub unsafe extern "system" fn driver_entry(
    driver: &mut DRIVER_OBJECT,
    registry_path: PCUNICODE_STRING,
) -> NTSTATUS {

//////////////////////////
// PARTIE 1 (EXERCICE 1)//
//////////////////////////
//
// La première etape de notre exercice rootkit consiste à créer un device contactable depuis le userland
// pour cela vous allez d'abord créer un device en utilisant la fonction IoCreateDevice puis vous allez 
// créer un lien symbolique vers ce device en utilisant la fonction IoCreateSymbolicLink
//
// let status = IoCreateDevice(
//                              DriverObject, DRIVER OBJECT PRESENT DES LA FONCTION DRIVER_ENTRY
//                              DeviceExtensionSize,
//                              DeviceName, NOM DU DEVICE AU FORMAT UNICODE
//                              DeviceType,
//                              DeviceCharacteristics,
//                              Exclusive,
//                              DeviceObject POINTEUR QUI SERA REMPLI PAR LA FONCTION 
//                              );  
// 
// Comme précisé dans le cours, inutile de s'attarder sur tout les arguments de la fonction IoCreateDevice, seul les arguments DriverObject, 
// DeviceName et DeviceObject sont intéressant
//
// REMINDER : N'oubliez pas que le nom du device doit être une chaine unicode ! Une methode to_unicode() est disponible pour vous aider
// à convertir un literal en UNICODE_STRING


// let status = IoCreateSymbolicLink(
//                                  SymbolicLinkName, NOM EXPOSE
//                                  DeviceName
//                                  );
//
// Notez bien le nom du lien symbolique, il vous sera ensuite utile pour votre client userland




// Les fonctions majeurs sont ici préconfiguré pour vous, vous n'avez pas besoin de les modifier
// vous aurez neanmoins besoin d'ecrire les fonctions major_function_create et major_function_device_control
    (*driver).MajorFunction[IRP_MJ_CREATE as usize] = Some(major_function_create);
    (*driver).MajorFunction[IRP_MJ_DEVICE_CONTROL as usize] = Some(major_function_device_control);


// La fonction driver_unload est ici préconfiguré pour vous, vous n'avez pas besoin de la modifier
    (*driver).DriverUnload = Some(driver_exit);

    STATUS_SUCCESS
}

// Function called when the driver is unloaded (on exit)
extern "C" fn driver_exit(driver: *mut DRIVER_OBJECT) {

    println!("Exiting driver!");

//////////////////////////
// PARTIE 2 (EXERCICE 1)//
//////////////////////////
//
// Avant de s'interesser à la suite des fonctiosn rootkit, il ne faut pas oublier de nettoyer les devices et les liens symboliques
// que vous avez créé dans la fonction driver_entry lorsque la fonction driver_exit est appelé
//
// Pour cela vous allez utiliser les fonctions IoDeleteDevice et IoDeleteSymbolicLink
//
//   
}

unsafe extern "C" fn major_function_create(_device: PDEVICE_OBJECT, pirp: PIRP) -> NTSTATUS {
    // Cette fonction est appelé lorsqu'un processus essaye d'ouvrir un handle vers le device
    // Comme dis durant le cours, il n'y a pas grand chose d'utile à faire ici. neanmoins
    // il peut être intéressant de print un message pour savoir si le device est bien contacté
    // avec la fonction CreateFileW depuis le userland
    // Exemple : println!("Device opened!");
    STATUS_SUCCESS
}

unsafe extern "C" fn major_function_device_control(_device: PDEVICE_OBJECT, pirp: PIRP) -> NTSTATUS {

//////////////////////////
// PARTIE 5 (EXERCICE 1)//
//////////////////////////
//
// La fonction major_function_device_control est appelé lorsque le client envoie un ordre
// au driver. Si vous avez completer les exercices précédents, vous devriez avoir un client
// capable d'envoyer un IRP avec un IOCTL code spécifique et un pointeur vers un PID.
//
//
// Pour ce qui est de l'extraction des informations de l'IRP, nous avons déja écrit la fonction
// IoGetCurrentIrpStackLocation.
    let stack = IoGetCurrentIrpStackLocation(pirp);
    let ioctl = (*stack).Parameters.DeviceIoControl.IoControlCode;

    
// Maitnenant que vous avez le IO Control code, vous pouvez filtrer si le code reçu correspond
// bien à un code pour cacher un processus et extraire le PID du processus à cacher
// Pour cela vous pouvez utiliser la constante IOCTL_PROCESS_HIDE_REQUEST .
// si vous avez besoin de debugger le fonctionnement de votre driver, vous pouvez placer 
// des println! pour afficher les valeurs des variables que vous manipulez dans windbg
    



}



pub unsafe fn IoGetCurrentIrpStackLocation(irp: PIRP) -> PIO_STACK_LOCATION {
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


