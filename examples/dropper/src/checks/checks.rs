use winapi::um::winnt::OSVERSIONINFOEXW;
use std::fs;
use std::io::Read;
use std::mem::zeroed;
use std::net::ToSocketAddrs;
use std::time::{SystemTime, UNIX_EPOCH};
use std::process::Command;
use std::{thread, time};
use sysinfo::{System, SystemExt, ComponentExt, ProcessExt};
use pnet::datalink;

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

        // Check MAC addresses for VM patterns
    if check_mac_addresses() {
        println!("MAC address indicates a VM ❌");
        return false;
    } else {
        println!("MAC address does not indicate a VM ✔️");
    }

    // Check CPU temperature
    if check_cpu_temperature() {
        println!("CPU temperature detected ✔️");
    } else {
        println!("No CPU temperature detected, likely a virtual machine ❌");
        return false;
    }

    // Check for suspicious processes
    if check_suspicious_processes() {
        println!("Suspicious processes detected ❌");
        return false;
    } else {
        println!("No suspicious processes detected ✔️");
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

// Check if the MAC address indicates a VM
fn check_mac_addresses() -> bool {
    let vm_mac_prefixes = vec![
        "08:00:27", // VirtualBox
        "00:0C:29", "00:1C:14", "00:50:56", "00:05:69", // VMware
    ];
    for iface in datalink::interfaces() {
        if let Some(mac) = iface.mac {
            let mac_str = mac.to_string().to_uppercase();
            for prefix in &vm_mac_prefixes {
                if mac_str.starts_with(prefix) {
                    println!("Detected VM MAC address: {}", mac_str);
                    return true;
                }
            }
        }
    }
    false
}

// Check the CPU temperature if not available is a VM
fn check_cpu_temperature() -> bool {
    let mut system = System::new_all();
    system.refresh_all();
    for component in system.components() {
        if let Some(temp) = component.temperature() {
            println!("CPU Temperature: {}°C", temp);
            return true;
        }
    }
    false
}

// Check for non needed processes
fn check_suspicious_processes() -> bool {
    let suspicious_processes = vec!["ollydbg.exe", "wireshark.exe", "procmon.exe", "ida.exe", "x64dbg.exe"];
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
