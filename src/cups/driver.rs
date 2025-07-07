use serde::Deserialize;
use std::process::Command;

#[derive(Debug,Default, Clone, Deserialize)]
pub struct Driver{
    pub value: String,
}

pub fn get_all_drivers() -> Vec<Driver> {
    let output = Command::new("lpinfo")
        .arg("-m")
        .output().expect("Failed to execute lpstat command check if CUPS is installed");
    let stdout = String::from_utf8(output.stdout).expect("Failed to convert output to string");
    let drivers:Vec<Driver> = stdout.lines()
    .filter_map(|input| {
        let value = input.trim().to_string();
        Some(Driver{
            value,
        })
    })
    .collect();
    drivers 
}
