use std::io::Write;

pub fn delete_last_line() {
    print!("\x1b[1A");
    print!("\x1b[2K");
    std::io::stdout().flush().unwrap();
}