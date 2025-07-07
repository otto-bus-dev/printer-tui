use serde::Deserialize;
use std::process::Command;

#[derive(Debug,Default, Clone, Deserialize)]
pub struct Printer{
    pub name : String,
    pub options : Vec<String>,
}
pub fn get_all_printers() -> Vec<Printer> {
    let output = Command::new("lpstat")
        .arg("-e")
        .output().expect("Failed to execute lpstat command check if CUPS is installed");
    let printer_names= String::from_utf8(output.stdout).expect("Failed to convert output to string");
    let enabled_printers:Vec<Printer> = printer_names.split('\n')
        .filter_map(|name| {
            let printer_name = name.trim();
            if printer_name.is_empty() {
                return None;
            }
            let options_output = Command::new("lpoptions")
                .arg("-d")
                .arg(printer_name)
                .output()
                .expect(format!(
                        "Failed to execute lpstat command check if CUPS is installed v {}",
                        printer_name
                    ).as_str());

            let line= String::from_utf8(options_output.stdout).expect("Failed to convert output to string");
            let options: Vec<String> = line.split(' ').map(|option| option.to_string()).collect();
            
            Some(Printer {
                name: printer_name.to_string(),
                options 
            })
        })
        .collect();
    enabled_printers
}

pub fn create_printer(name: String, device: String, driver: String) {

    Command::new("lpadmin")
        .arg("-p")
        .arg(name.clone())
        .arg("-E")
        .arg("-v")
        .arg(device.clone())
        .arg("-m")
        .arg(driver.clone())
        .output().expect("Failed to execute lpstat command check if CUPS is installed");
    
}
pub fn remove_printer(name: &str) {
    Command::new("lpadmin")
        .arg("-x")
        .arg(name)
        .output().expect("Failed to execute lpstat command check if CUPS is installed");
}


