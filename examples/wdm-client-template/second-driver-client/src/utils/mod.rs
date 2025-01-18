mod privileges;

use std::process;
use crate::process_privesc;

pub fn self_privesc() {
    let pid = process::id();
    process_privesc(pid);
}

pub fn set_backup_privilege(){
    privileges::set_backup_privilege();
}