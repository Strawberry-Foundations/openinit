use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub fn delete_last_line() {
    print!("\x1b[1A");
    print!("\x1b[2K");
    std::io::stdout().flush().unwrap();
}

pub fn get_os_name() -> String {
    if let Ok(file) = File::open("/etc/os-release") {
        let reader = BufReader::new(file);
        
        let mut name: String = String::new();

        for line in reader.lines().map_while(Result::ok) {
            if line.starts_with("PRETTY_NAME=") {
                name = line.split('=').nth(1).unwrap_or("").trim_matches('"').parse().unwrap();
                break;
            }
        }
        
        name
    } else {
        String::from("Failed to open /etc/os-release")
    }
}