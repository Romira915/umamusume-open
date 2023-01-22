#![windows_subsystem = "windows"]

use std::process::{exit, Command};

fn main() {
    let mut command = Command::new("cmd")
        .arg("/c")
        .arg("start")
        .arg("dmmgameplayer://play/GCL/umamusume/cl/win")
        .spawn()
        .expect("Failed to start command.");
    command.wait().expect("Failed to wait for command.");

    exit(0);
}
