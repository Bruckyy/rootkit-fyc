
use alloc::string::{String, ToString};
use wdk_sys::LIST_ENTRY;
use wdk_sys::HANDLE;

mod shadowed_process;
use shadowed_process::ShadowedProcess;

extern "system" {
    // Déclarer ici les fonction externes (Les API Windows)
}

//////////////////////////
// PARTIE 2 (EXERCICE 2)//
//////////////////////////

pub fn shadow_process(target_pid: u32) -> Result<bool,String> {
    // Fonction itérant sur la liste doublement chaînée afin de trouver le processus à cacher
    // Retourne Ok(true) en cas de réussite ou Err("Message d'erreur".to_string()) en cas d'échec

    // Utiliser la structure ShadowedProcess afin de faciliter la gestion de la structure EPROCESS et la navigation à travers la liste de processus
    // Pour identifier le processus à shadow il vous faudra choisir un attribut unique de la structure EPROCESS (plusieurs possibilités)
    // Une fois que vous avez un pointeur sur la structure EPROCESS du processus à cacher, appelez la fonction remove_links() qui s'occupera de la tâche finale

    return Err("Process not found".to_string());
}

//////////////////////////
// PARTIE 3 (EXERCICE 2)//
//////////////////////////

unsafe fn remove_links(current: *mut LIST_ENTRY) -> Result<bool,String> {
    //
    // Voici une représentation visuelle de ce que la fonction effectue
    // 
    // BEFORE OPERATION
    // [Previous Process]   <-->   [Target Process]   <-->   [Next Process]
    //         ^                                                  ^
    //         |                                                  |
    //     (*Target).Blink                                    (*Target).Flink
    //
    // AFTER OPERATION             
    //                              ---------
    //                              ↓       |
    //                [Target Process] ------
    //
    // [Previous]   <----------------->   [Next]
    //     (*Previous).Flink       (*Next).Blink
    //
    //
    // Cette fonction altère les pointeurs (Flink et Blink) des structures du processus précédent et suivant de notre cible à faire disparaitre.
    //
    // Il vous faut :
    // - Faire pointer le Blink du processus suivant sur la LIST_ENTRY du processus précédent  
    // - Faire pointer le Flink du processus précédent sur la LIST_ENTRY du processus suivant
    // - Faire pointer Flink et Blink de notre processus cible sur sa propre LIST_ENTRY de sorte à créer une boucle
    //
    // Comme pour la fonction shadow_process() retourner une valeur cohérente 
    // Vérifiez bien la validité des pointeurs, en mode noyau chaque erreur est fatale.

    return Ok(true);
}