use std::env;
use std::fs::File;
use std::io::Read;

mod parser;
use parser::Parser;
mod code;
use code::Code;


fn print_usage() {
    println!("Usage: command <filename>");
}

fn main() {
    let file_name = match env::args().nth(1) {
        Some(file_name) => file_name,
        None => {
            println!("Error: File name is not exist.");
            print_usage();
            return;
        }
    };

    let mut file = match File::open(&file_name) {
        Ok(file) => file,
        Err(_) => {
            println!("Error: {} is not exist.", &file_name);
            print_usage();
            return;
        }
    };

    let mut asm = String::new();
    let _ = file.read_to_string(&mut asm);

    println!("{}", asm);
}
