//////////////////////////
// PARTIE 1 (EXERCICE 2)//
//////////////////////////

use wdk_sys::LIST_ENTRY;
use wdk_sys::PVOID;
// Identifier les offset des champs qui nous intéressent (à récupérer sur windbg ou vergilius project)
const ACTIVE_PROCESS_LINKS_OFFSET: usize = 0x0;
const PID_OFFSET: usize = 0x0;


#[derive(Clone)]
pub struct ShadowedProcess {
    // La structure représentant le procesus à faire disparaître, doit contenir à minima trois attributs :

    // Un pointeur sur la structure EPROCESS du processus
    // Un pointeur sur le PID du processus
    // Un pointeur sur la active_process_links (LIST_ENTRY)

    // Vous êtes libre d'en ajouter d'autres si vous trouvez cela pertinent.
}

impl ShadowedProcess {
    pub unsafe fn from_eprocess(eprocess: PVOID) -> Self {
        // Permet de créer une structure ShadowedProcess à partir d'un pointeur sur EPROCESS
        // Affecter les champs pid, active_process_links (LIST_ENTRY) à l'aide des offset définit plus tôt
        return ShadowedProcess{}
    }

    pub unsafe fn next(&self) -> Self {
        // Renvoie la structure ShadowedProcess du processus suivant dans la liste chaînée.


        return self.clone() // À SUPPRIMER (uniquement nécessaire à la compilation initiale)
    }

}