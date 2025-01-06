use winapi::um::winnt::OSVERSIONINFOEXW;
use std::fs;
use std::io::Read;
use std::mem::zeroed;
use std::net::ToSocketAddrs;
use std::time::{SystemTime, UNIX_EPOCH};
use std::process::Command;
use std::{thread, time};


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

    // Check internet connectivity
    if check_internet() {
        println!("Internet ✔️");
    } else {
        println!("Internet ❌");
        return false;
    }

    // Check if machine is VM or container
    if check_virtual_machine() {
        println!("VM/Container ✔️");
    } else {
        println!("VM or Container detected ❌");
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
    // if let Some(uptime) = get_uptime() {
    //     if uptime > 3600 {
    //         println!("Uptime: {} seconds ✔️", uptime);
    //     } else {
    //         println!("Uptime: {} seconds ❌ (Less than 1 hour)", uptime);
    //         return false;
    //     }
    // } else {
    //     println!("Failed to get uptime ❌");
    //     return false;
    // }

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

    if (get_num_threads() %2) !=0 {
        return false
    }

    if !sleep_test() {
        return false; 
    }

    if let Ok(output) = cpuid {
        let output_str = String::from_utf8_lossy(&output.stdout).to_lowercase();
        if output_str.contains("virtual") || output_str.contains("vmware") || output_str.contains("hyper-v") {
            return false;
        }
    }

    if let Ok(mut file) = fs::File::open("/proc/self/cgroup") {
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap_or_default();
        if contents.contains("docker") || contents.contains("lxc") {
            return false;
        }
    }

    true
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

fn get_num_threads() -> usize {
    let num_cpus = thread::available_parallelism()
    .map(|n| n.get())
    .unwrap_or(1); // Fallback to 1 in case of error

    num_cpus
}

// Try to identify if the sleep is not skipped
fn sleep_test() -> bool {
    let duration = time::Duration::from_millis(3000);
    let now = time::Instant::now();
    thread::sleep(duration);
    if now.elapsed() >= duration {
        return true;
    }
    false
}

// fn get_uptime() -> Option<u64> {
//     if let Ok(duration) = SystemTime::now().duration_since(UNIX_EPOCH) {
//         let boot_time = Command::new("wmic")
//             .args(["os", "get", "LastBootUpTime"])
//             .output()
//             .ok()?;

//         let output = String::from_utf8_lossy(&boot_time.stdout);
//         let boot_time_str = output.lines().nth(1)?.trim();
//         let boot_time_secs = boot_time_str.parse::<u64>().ok()?;
//         return Some(duration.as_secs() - boot_time_secs);
//     }
//     None
// }