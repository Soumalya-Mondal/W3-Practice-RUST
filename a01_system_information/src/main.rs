// 1. Write a Rust program that gathers system information such as the Rust version, HOSTNAME, and IP-Details.

use std::process::Command;
fn main() {
    // Get Rust version
    let rustc_version_output = Command::new("rustc")
        .arg("--version")
        .output()
        .expect("Failed To Execute Command");
    let rustc_version: String = String::from_utf8_lossy(&rustc_version_output.stdout).trim().to_uppercase().to_string();
    println!("RUST Compiler Version: {}", rustc_version);

    // Get HOSTNAME
    let hostname_output = Command::new("hostname")
        .output()
        .expect("Failed To Execute Command");
    let hostname: String = String::from_utf8_lossy(&hostname_output.stdout).trim().to_uppercase().to_string();
    println!("System HOSTNAME: {}", hostname);

    // Get IP-Details
    let ip_config_output = Command::new("ipconfig")
        .output()
        .expect("Failed To Execute Command");
    let ip_config: String = String::from_utf8_lossy(&ip_config_output.stdout).trim().to_string();
    println!("IP Config: {}", ip_config);
}