use std::ffi::c_void;

use crate::constants::IOCTL_PROCESS_HIDE_REQUEST;

use windows::core::w;
use windows::Win32::Foundation::{HANDLE, STATUS_SUCCESS, STATUS_UNSUCCESSFUL};
use windows::Win32::Storage::FileSystem::CreateFileW;
use windows::Win32::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES;
use windows::Win32::Storage::FileSystem::{CREATE_ALWAYS, OPEN_EXISTING};
use windows::Win32::Storage::FileSystem::{
    FILE_GENERIC_READ, FILE_GENERIC_WRITE, FILE_SHARE_READ, FILE_SHARE_WRITE,
};
use windows::Win32::System::IO::DeviceIoControl;




pub unsafe fn contact_driver() -> HANDLE {

//////////////////////////
// PARTIE 3 (EXERCICE 1)//
//////////////////////////
//
// La fonction contact_driver sert à ouvrir le device exposé par le driver en utilisant le lien
// symbolique créé dans l'exercice 1. Vous devez ouvrir le device en lecture et écriture.
//
// Pour cela vous allez utiliser la fonction CreateFileW en passant le nom du device en argument.
// 
// let handle = CreateFileW(
//                          lpFileName, LE NOM DU LIEN SYMBOLIQUE EN UNICODE
//                          dwDesiredAccess,
//                          dwShareMode,
//                          lpSecurityAttributes,
//                          dwCreationDisposition,
//                          dwFlagsAndAttributes, NE PAS OUBLIER QU'ON OUVRE UN OBJET DEJA EXISTANT
//                          hTemplateFile
//                          );
//
//
//
}

pub unsafe fn hide_process(driver_handle: HANDLE, pid: u32) {

//////////////////////////
// PARTIE 3 (EXERCICE 1)//
//////////////////////////
//
// La fonction hide_process sert à envoyer une requête au driver pour cacher un processus.
// Pour cela vous allez utiliser la fonction DeviceIoControl en passant le IO Control code
// IOCTL_PROCESS_HIDE_REQUEST et le PID du processus à cacher.
//
// DeviceIoControl(
//                 hDevice, HANDLE DU DEVICE
//                 dwIoControlCode, IOCTL_PROCESS_HIDE_REQUEST
//                 lpInBuffer, POINTEUR VERS LE PID
//                 nInBufferSize, TAILLE DU PID
//                 lpOutBuffer, 
//                 nOutBufferSize, 
//                 lpBytesReturned,
//                 lpOverlapped
//                 );
//
// Comme précisé dans le cours les arguments importants sont hDevice, dwIoControlCode, lpInBuffer et nInBufferSize
// Les autres arguments peuvent être mis à None, pointeur nulle ou 0.
// Si vous voulez recuperer une réponse de la part de votre driver, vous pouvez utiliser les arguments lpOutBuffer, 
// nOutBufferSize et lpBytesReturned.
// 
// EXTRA : Essayer de prendre en compte la réponse du driver et de retourner un resultat en conséquence

// 

    let pid_ptr: *mut c_void = pid as *mut c_void;
    let pid_size = std::mem::size_of::<u32>() as u32;
    let mut output_buffer: [u8; 4] = [0; 4];
    let mut bytes_returned: u32 = 0;

    let result = DeviceIoControl(
        driver_handle,
        IOCTL_PROCESS_HIDE_REQUEST,
        Some(pid_ptr),
        pid_size,
        Some(output_buffer.as_mut_ptr() as *mut c_void),
        output_buffer.len() as u32,
        Some(&mut bytes_returned),
        None,
    );

    // TODO: maybe enhance this
    match output_buffer[0] {
        1 => println!("Successfully shadowed process PID: {:?}", pid),
        0 => println!("Process not found"),
        _ => println!("Uknown response"),
    }
}
