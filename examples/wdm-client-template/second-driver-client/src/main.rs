use clap::Parser;



use windows::core::w;
mod constants;

mod kernel_interface;
use kernel_interface::contact_driver;
use kernel_interface::hide_process;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[arg(short, long)]
    command: u32,
}

fn main() {
    let args = Args::parse();

    match args.command {
        0 => println!("No process to hide"),
        _ => {
            println!("Hiding process");
            unsafe {
                let driver_handle = contact_driver("\\\\.\\MyDevice");
                let pid = args.command;
                hide_process(driver_handle, pid);
            }
        }
    }
}
