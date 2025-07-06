use serde::Deserialize;
use std::process::Command;

#[derive(Debug,Default, Clone, Deserialize)]
pub struct Device{
    pub value : String
}

pub fn get_all_devices() -> Vec<Device> {
    let output = Command::new("lpinfo")
        .arg("-v")
        .output().expect("Failed to execute lpstat command check if CUPS is installed");
    let stdout = String::from_utf8(output.stdout).expect("Failed to convert output to string");
    let available_printers:Vec<Device> = stdout.lines()
    .filter_map(|input| {
        let line = input.trim();
        if line.is_empty() {
            return None;
        }
        let words:Vec<&str> = line.split(' ').collect();
        if words.len() < 2 { 
            None
        }else{
            Some(Device{
                value:words[1].to_string(),
            })
        }
    })
    .collect();
    available_printers
}

