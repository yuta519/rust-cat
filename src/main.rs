use std::fs::read_to_string;

fn main() {
    match std::env::args().nth(1) {
        Some(path) => cat_cmd(path),
        None => println!("Please specify the file path at an argument!"),
    }
}

fn cat_cmd(path: String) {
    match read_to_string(path) {
        Ok(content) => print!("{}", content),
        Err(reason) => print!("{}", reason),
    }
}
