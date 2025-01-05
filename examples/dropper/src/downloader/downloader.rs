use std::io;
use std::fs::File;

pub fn download_payload() -> bool {

    let payload = "https://raw.githubusercontent.com/microsoft/windows-rs/refs/heads/master/license-apache-2.0"; // Test file for testing purposes

    let resp = match reqwest::blocking::get(payload) {
        Ok(resp) => {
            if resp.status().is_success(){
                resp
            } else {
                return false
            }
        },
        Err(_) => return false
    };

    let body = match resp.text() {
        Ok(body) => body,
        Err(_) => return false 
    };

    let mut file = match File::create("test.txt") { // Maybe we will choose C:\\Windows\\Temp as dest
        Ok(file) => file,
        Err(_) => return false
    };

    match io::copy(&mut body.as_bytes(), &mut file){
        Ok(_) => true,
        Err(_) => false
    }
}