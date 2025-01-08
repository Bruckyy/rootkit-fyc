mod checks;
mod downloader;

use crate::checks::checks::{perform_checks};
use crate::downloader::downloader::download_payload;
use std::process;


#[tokio::main]
async fn main() {

    if perform_checks() {
        println!("[+] All Checks OK ✔️");
        if let Err(e) = download_payload().await {
            println!("[x] Failed to download payload ❌: {}", e);
            process::exit(1);
        } else {
            println!("[+] Payload downloaded successfully ✔️")
            
        }
    } else {
        println!("[x] Checks Failed aborting deployment ❌");
    }

}