use winapi::um::winnt::OSVERSIONINFOEXW;
use std::fs;
use std::io::Read;
use std::mem::zeroed;
use std::net::ToSocketAddrs;
use std::time::{SystemTime, UNIX_EPOCH};
use std::process::Command;
use winreg::enums::*;
use winreg::RegKey;

extern "system" {
    fn RtlGetVersion(lpVersionInformation: *mut OSVERSIONINFOEXW) -> i32;
}

pub fn perform_checks() -> bool {
    let os_info = get_version_info();

    // Check OS version
    if os_info.dwMajorVersion != 10 {
        println!("Version ❌");
        return false;
    } else {
        println!("Version ✔️");
    }

    // Check if Windows version is 22H2
    if get_win_build() {
        println!("Windows version is 22H2 ✔️");
    } else {
        println!("Windows version is not 22H2 ❌");
        return false;
    }

    // Check internet connectivity
    if check_internet() {
        println!("Internet ✔️");
    } else {
        println!("Internet ❌");
        return false;
    }

    // Check if machine is VM or container
    if !check_virtual_machine() {
        println!("Machine is physical ✔️");
    } else {
        println!("Machine is a VM or container ❌");
        return false;
    }

    // Check user activity
    if check_user_activity() {
        println!("User activity detected ✔️");
    } else {
        println!("No user activity ❌");
        return false;
    }

    // Check uptime
    if let Some(uptime) = get_uptime() {
        if uptime > 3600 {
            println!("Uptime: {} seconds ✔️", uptime);
        } else {
            println!("Uptime: {} seconds ❌ (Less than 1 hour)", uptime);
            return false;
        }
    } else {
        println!("Failed to get uptime ❌");
        return false;
    }

    true
}

fn get_version_info() -> OSVERSIONINFOEXW {
    unsafe {
        let mut os_info: OSVERSIONINFOEXW = zeroed();
        os_info.dwOSVersionInfoSize = std::mem::size_of::<OSVERSIONINFOEXW>() as u32;
        RtlGetVersion(&mut os_info);
        os_info
    }
}

// Search with win API maybe later
fn get_win_build() -> bool {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    if let Ok(key) = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion") {
        if let Ok(build_lab) = key.get_value::<String>("BuildLabEx") {
            return build_lab.contains("19045");
        }
    }
    false
}

fn check_internet() -> bool {
    let target_dom = "google.com:443";
    match target_dom.to_socket_addrs() {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn check_virtual_machine() -> bool {
    let cpuid = Command::new("wmic")
        .args(["computersystem", "get", "model"])
        .output();

    if let Ok(output) = cpuid {
        let output_str = String::from_utf8_lossy(&output.stdout).to_lowercase();
        if output_str.contains("virtual") || output_str.contains("vmware") || output_str.contains("hyper-v") {
            return true;
        }
    }

    if let Ok(mut file) = fs::File::open("/proc/self/cgroup") {
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap_or_default();
        if contents.contains("docker") || contents.contains("lxc") {
            return true;
        }
    }

    false
}

fn check_user_activity() -> bool {
    let default_profiles = vec!["Default", "Public", "Default User", "All Users"];
    if let Ok(entries) = fs::read_dir("C:\\Users") {
        for entry in entries {
            if let Ok(entry) = entry {
                let folder_name = entry.file_name().into_string().unwrap_or_default();
                if !default_profiles.contains(&folder_name.as_str()) {
                    return true;
                }
            }
        }
    }
    false
}

fn get_uptime() -> Option<u64> {
    if let Ok(duration) = SystemTime::now().duration_since(UNIX_EPOCH) {
        let boot_time = Command::new("wmic")
            .args(["os", "get", "LastBootUpTime"])
            .output()
            .ok()?;

        let output = String::from_utf8_lossy(&boot_time.stdout);
        let boot_time_str = output.lines().nth(1)?.trim();
        let boot_time_secs = boot_time_str.parse::<u64>().ok()?;
        return Some(duration.as_secs() - boot_time_secs);
    }
    None
}