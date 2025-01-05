use winapi::um::winnt::OSVERSIONINFOEXW;
use std::mem::zeroed;
use std::net::ToSocketAddrs;

extern "system" {
    fn RtlGetVersion(lpVersionInformation: *mut OSVERSIONINFOEXW) -> i32;
}

pub fn perfom_checks() -> bool {
    let os_info = get_version_info();

    if os_info.dwMajorVersion != 10 { // We need to check BuildNumber too os_info.dwBuildNumber
        println!("Version ❌");
        return false
    } else {
        println!("Version ✔️")
    }


    if check_internet() {
        println!("Internet ✔️")
    } else {
        println!("Internet ❌");
        return false
    }

    true
}


fn get_version_info() -> OSVERSIONINFOEXW {
    unsafe {
        let mut os_info: OSVERSIONINFOEXW = zeroed(); // https://learn.microsoft.com/fr-fr/windows/win32/api/winnt/ns-winnt-osversioninfoexw
        os_info.dwOSVersionInfoSize = std::mem::size_of::<OSVERSIONINFOEXW>() as u32;

        RtlGetVersion(&mut os_info);

        os_info
    }
}

// Perform a dns request to google.com
fn check_internet() -> bool {
    
    let target_dom = "google.com:443";

    match target_dom.to_socket_addrs() {
        Ok(_) => {
            true
        },
        Err(_) => {
            false
        }
    }
}


// TODO: Write more checks (example: Virtual Machine detection)