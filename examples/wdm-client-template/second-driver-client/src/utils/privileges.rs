use windows::{
    Win32::{
        Security::{
            AdjustTokenPrivileges, LookupPrivilegeValueW, TOKEN_PRIVILEGES, SE_PRIVILEGE_ENABLED, TOKEN_ADJUST_PRIVILEGES, TOKEN_QUERY, LUID_AND_ATTRIBUTES, TOKEN_PRIVILEGES_ATTRIBUTES
        },
        System::Threading::{
            OpenProcessToken, GetCurrentProcess
        },
        Foundation::{
            HANDLE, GetLastError, ERROR_NOT_ALL_ASSIGNED, LUID
        }
    },
    core::PCWSTR,
};

fn set_privilege(
    h_token: HANDLE,        // Handle du token d'accès
    privilege: &str,        // Nom du privilège à activer/désactiver
    enable_privilege: bool, // Activer ou désactiver le privilège
) -> Result<(), String> {
    unsafe {
        // Convertir le nom du privilège en UTF-16
        let privilege_wide: Vec<u16> = privilege.encode_utf16().chain(Some(0)).collect();
        let privilege_name = PCWSTR(privilege_wide.as_ptr());

        // Initialiser les structures nécessaires
        let mut luid = LUID::default();
        match LookupPrivilegeValueW(PCWSTR::null(), privilege_name, &mut luid) {
            Ok(_) => {},
            Err(_) => {
                return Err(format!("LookupPrivilegeValueW a échoué avec l'erreur : {:?}", GetLastError()));
            }
        }

        // Configurer TOKEN_PRIVILEGES
        let mut tp = TOKEN_PRIVILEGES {
            PrivilegeCount: 1,
            Privileges: [LUID_AND_ATTRIBUTES {
                Luid: luid,
                Attributes: if enable_privilege {
                    SE_PRIVILEGE_ENABLED
                } else {
                    TOKEN_PRIVILEGES_ATTRIBUTES(0)
                },
            }],
        };

        // Ajuster les privilèges
        match AdjustTokenPrivileges(h_token, false, Some(&mut tp), 0, None, None) {
            Ok(_) => {},
            Err(_) => {
                return Err(format!("AdjustTokenPrivileges a échoué avec l'erreur : {:?}", GetLastError()));
            }
        }

        // Vérifier si tous les privilèges ont été assignés correctement
        if GetLastError() == ERROR_NOT_ALL_ASSIGNED {
            return Err("Le token ne dispose pas du privilège spécifié.".to_string());
        }

        Ok(())
    }
}

pub fn set_backup_privilege(){
    let backup_privilege_name = "SeBackupPrivilege";

    // Exemple d'utilisation
    unsafe {
        let mut token_handle: HANDLE = HANDLE::default();

        // Ouvrir le token de processus
        match OpenProcessToken(GetCurrentProcess(), TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY, &mut token_handle) {
            Ok(_) => {},
            Err(err) => {
                eprintln!("OpenProcessToken a échoué avec l'erreur : {}", err);
                return;
            }
        }

        // Activer le privilège SE_BACKUP_NAME
        match set_privilege(token_handle, backup_privilege_name, true) {
            Ok(_) => println!("Privilège {} activé avec succès.", backup_privilege_name),
            Err(e) => eprintln!("Erreur : {}", e),
        }
    }
}