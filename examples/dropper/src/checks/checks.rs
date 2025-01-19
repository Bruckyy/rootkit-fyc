use winapi::um::winnt::OSVERSIONINFOEXW;
use std::fs;
use std::mem::zeroed;
use std::net::ToSocketAddrs;
use std::process::Command;
use std::{thread, time};
use sysinfo::{System, SystemExt, ProcessExt};
use get_if_addrs::{get_if_addrs};

extern "system" {
    fn RtlGetVersion(lpVersionInformation: *mut OSVERSIONINFOEXW) -> i32;
}

pub fn perform_checks() -> bool {
    let os_info = get_version_info();


    // Check OS version
    if os_info.dwMajorVersion != 10 {
        println!("Version [X]");
        thread::sleep(time::Duration::from_secs(5));
        return false;
    } else {
        println!("Version [✓]");
    }

    // Check internet connectivity
    if check_internet() {
        println!("Internet [✓]");
    } else {
        println!("Internet [X]");
        thread::sleep(time::Duration::from_secs(5));
        return false;
    }

    // Check if machine is VM or container
    if check_virtual_machine() {
        println!("VM/Container [✓]");
    } else {
        println!("VM or Container detected [X]");
        thread::sleep(time::Duration::from_secs(5));
        return false;
    }

    // Check user activity
    if check_user_activity() {
        println!("User activity detected [✓]");
    } else {
        println!("No user activity [X]");
        thread::sleep(time::Duration::from_secs(5));
        return false;
    }

    // Check VM-specific files
    if check_vm_files() {
        println!("No VM files detected [✓]");
    } else {
        println!("VM files detected [X]");
        thread::sleep(time::Duration::from_secs(5));
        return false;
    }
    

    // Check MAC addresses for VM patterns
    if check_mac_addresses() {
        println!("MAC address indicates a VM [X]");
        thread::sleep(time::Duration::from_secs(5));
        return false;
    } else {
        println!("MAC address does not indicate a VM [✓]");
    }

    // Check for suspicious processes
    if check_suspicious_processes() {
        println!("Suspicious processes detected [X]");
        thread::sleep(time::Duration::from_secs(5));
        return false;
    } else {
        println!("No suspicious processes detected [✓]");
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

    true
}

fn check_user_activity() -> bool {
    let default_profiles = vec!["Default", "Public", "Default User", "All Users"]; // Default Windows profiles
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

// Check if the MAC address indicates a VM
fn check_mac_addresses() -> bool {
    let vm_mac_prefixes = vec![
        "08:00:27", // VirtualBox
        "00:0C:29", "00:1C:14", "00:50:56", "00:05:69", // VMware
    ];

    if let Ok(ifaces) = get_if_addrs() {
        for iface in ifaces {
            if let get_if_addrs::IfAddr::V4(addr) = iface.addr {
                let mac_option = addr.broadcast; // Get the MAC address
            if let Some(mac) = mac_option {
                let mac_str = mac.to_string().to_uppercase();
            for prefix in &vm_mac_prefixes {
                if mac_str.starts_with(prefix) {
                    println!("Detected VM MAC address: {}", mac_str);
                    return true;
                        }
                    }
                }
            }
        }
    }

    false
}

// Check for non needed processes
fn check_suspicious_processes() -> bool {
    let suspicious_processes = vec!["ollydbg.exe", "wireshark.exe", "procmon.exe", "ida.exe", "x64dbg.exe"]; // process names to check for
    let mut system = System::new_all();
    system.refresh_processes();
    for (_pid, process) in system.processes() {
        let name = process.name().to_lowercase();
        if suspicious_processes.iter().any(|&suspect| name.contains(suspect)) {
            println!("Detected suspicious process: {}", name);
            return true;
        }
    }
    false
}

fn check_vm_files() -> bool {
    let vm_files = vec![
        "C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs\\StartUp\\agent.pyw",
        "C:\\WINDOWS\\system32\\drivers\\vmmouse.sys",
        "C:\\WINDOWS\\system32\\drivers\\vmhgfs.sys",
        "C:\\WINDOWS\\system32\\drivers\\VBoxMouse.sys",
        "C:\\WINDOWS\\system32\\drivers\\VBoxGuest.sys",
        "C:\\WINDOWS\\system32\\drivers\\VBoxSF.sys",
        "C:\\WINDOWS\\system32\\drivers\\VBoxVideo.sys",
        "C:\\WINDOWS\\system32\\vboxdisp.dll",
        "C:\\WINDOWS\\system32\\vboxhook.dll",
        "C:\\WINDOWS\\system32\\vboxmrxnp.dll",
        "C:\\WINDOWS\\system32\\vboxogl.dll",
        "C:\\WINDOWS\\system32\\vboxoglarrayspu.dll",
        "C:\\WINDOWS\\system32\\vboxoglcrutil.dll",
        "C:\\WINDOWS\\system32\\vboxoglerrorspu.dll",
        "C:\\WINDOWS\\system32\\vboxoglfeedbackspu.dll",
        "C:\\WINDOWS\\system32\\vboxoglpassthroughspu.dll",
        "C:\\WINDOWS\\system32\\vboxservice.exe",
        "C:\\WINDOWS\\system32\\vboxtray.exe",
        "C:\\WINDOWS\\system32\\VBoxControl.exe",
    ];

    for file_path in vm_files {
        if fs::metadata(file_path).is_ok() {
            println!("Detected VM file: {}", file_path);
            return false;
        }
    }

    true
}
