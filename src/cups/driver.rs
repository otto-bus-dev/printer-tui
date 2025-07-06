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
        // if line.is_empty() {
        //     return None;
        // }
        // let words:Vec<&str> = line.split(' ').collect();
        //
        // let backend:Option<String> = if words.len() > 0{
        //     Some(words[0].to_string())
        // } else {
        //     None
        // };
        // let uri:Option<String> = if words.len() > 1{
        //     Some(words[1].to_string())
        // } else {
        //     None
        // };
        Some(Driver{
            value,
        })
    })
    .collect();
    drivers 
}
