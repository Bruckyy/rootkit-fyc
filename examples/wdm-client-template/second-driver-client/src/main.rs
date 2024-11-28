use clap::Parser;

use windows::core::w;
mod constants;

mod kernel_interface;
use kernel_interface::contact_driver;
use kernel_interface::hide_process;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "PID of the process to hide")]
    process: u32,
}

fn main() {
    let args = Args::parse();

    match args.process {
        0 => println!("No process to hide"),
        _ => {
            println!("Hiding process PID: {:?}", args.process);
            unsafe {
                let driver_handle = contact_driver("\\\\.\\MyDevice");
                let pid = args.process;
                hide_process(driver_handle, pid);
            }
        }
    }
}
