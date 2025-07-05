use serde::Deserialize;
use std::process::Command;
use std::io::Write;
use ratatui::layout::Rect;
#[derive(Debug,Default, Clone, Deserialize)]
pub struct Device{
    pub uri: Option<String>,
    pub backend: Option<String>,
}
#[derive(Debug,Default, Clone, Deserialize)]
pub struct Printer{
    pub name : String,
    pub uri: Option<String>,
    pub backend: Option<String>,
}
#[derive(Debug,Default, Clone, Deserialize)]
pub struct Driver{
    pub uri: Option<String>,
    pub backend: Option<String>,
}
impl Printer {
    pub fn get_all() -> Vec<Printer> {
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
                let output = Command::new("lpstat")
                    .arg("-v")                    
                    .arg(printer_name)
                    .output()
                    .expect(format!(
                            "Failed to execute lpstat command check if CUPS is installed v {}",
                            printer_name
                        ).as_str());

                let line= String::from_utf8(output.stdout).expect("Failed to convert output to string");
                let words: Vec<&str> = line.split(':').collect();
                let uri = words.get(2).map(|s| s.to_string());
                let backend = words.get(1).map(|s| s.to_string());

                Some(Printer {
                    name: printer_name.to_string(),
                    backend: backend,
                    uri: uri,
                })
            })
            .collect();
        enabled_printers
    }

    pub fn default() -> Printer {
        Printer {
            name: "New Printer".to_string(),
            uri: None,
            backend: None,
        }
    }

    pub fn create(name: String, uri: Option<String>, driver: Option<String>) {
        //+ uri.as_ref().expect("URI must be set for printer");
        // Command::new("lpadmin")
        //     .arg("-p")
        //     .arg(name)
        //     .arg("-E")
        //     .arg("-v")
        //     .arg(uri.as_ref().expect("URI must be set for printer"))
        //     .arg("-m")
        //     .arg(driver.as_ref().expect("Backend must be set for printer"))
        //     .output().expect("Failed to execute lpstat command check if CUPS is installed");
        Command::new("lpadmin")
            .arg("-p")
            .arg("test_printer")
            .arg("-E")
            .arg("-v")
            .arg("lpd://BRW30C9AB7676AB/BINARY_P1")
            .arg("-m")
            .arg("brother_mfcj5330dw_printer_en.ppd")
            .output().expect("Failed to execute lpstat command check if CUPS is installed");
        
        
    }

    pub fn remove(&self) {
        Command::new("lpadmin")
            .arg("-r")
            .output().expect("Failed to execute lpstat command check if CUPS is installed");
    }
    
}

impl Driver{

    pub fn get_all() -> Vec<Driver> {
        let output = Command::new("lpinfo")
            .arg("-m")
            .output().expect("Failed to execute lpstat command check if CUPS is installed");
        let stdout = String::from_utf8(output.stdout).expect("Failed to convert output to string");
        let drivers:Vec<Driver> = stdout.lines()
        .filter_map(|input| {
            let line = input.trim();
            if line.is_empty() {
                return None;
            }
            let words:Vec<&str> = line.split(' ').collect();
            
            let backend:Option<String> = if words.len() > 0{
                Some(words[0].to_string())
            } else {
                None
            };
            let uri:Option<String> = if words.len() > 1{
                Some(words[1].to_string())
            } else {
                None
            };

                Some(Driver{
                    backend: backend,
                    uri: uri,
                })
        })
        .collect();
        drivers 
    }

    
}

impl Device{

    pub fn get_all() -> Vec<Device> {
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
            let values:Vec<&str> = if words.len() > 1{
                words[1].split(':').collect()
            } else {
                vec![] 
            };
            let backend:Option<String> = if values.len() > 0{
                Some(values[0].to_string())
            } else {
                None
            };
            let uri:Option<String> = if values.len() > 1{
                Some(values[1].to_string())
            } else {
                None
            };

                Some(Device{
                    backend: backend,
                    uri: uri,
                })
        })
        .collect();
        available_printers
    }

}
