mod checks;
use crate::checks::checks::{perfom_checks};


fn main() {

    if perfom_checks() {
        println!("[!] All Checks OK ✔️");
    } else {
        println!("[!] Checks Failed ❌");
    }
}