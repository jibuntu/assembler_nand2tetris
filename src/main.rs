use std::env;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

mod parser;
use parser::Parser;
use parser::CommandType;
mod code;
use code::Code;


fn print_usage() {
    println!("Usage: command <filename>");
}

/// 数字を16bitのバイナリへ変換する
fn number_to_16bin(mut number: i16) -> [usize;16] {
    let mut bin = [0_usize; 16];

    for i in 0..16 {
        bin[15 - i] = (number & 1_i16) as usize;
        number >>= 1;
    }

    bin
}
#[test]
fn test_number_to_16bin() {
    assert_eq!(number_to_16bin(0), [0,0,0,0,  0,0,0,0,  0,0,0,0,  0,0,0,0]);
    assert_eq!(number_to_16bin(1), [0,0,0,0,  0,0,0,0,  0,0,0,0,  0,0,0,1]);
    assert_eq!(number_to_16bin(2), [0,0,0,0,  0,0,0,0,  0,0,0,0,  0,0,1,0]);
    assert_eq!(number_to_16bin(7), [0,0,0,0,  0,0,0,0,  0,0,0,0,  0,1,1,1]);
    assert_eq!(number_to_16bin(-1), [1,1,1,1,  1,1,1,1,  1,1,1,1,  1,1,1,1]);
    assert_eq!(number_to_16bin(-2), [1,1,1,1,  1,1,1,1,  1,1,1,1,  1,1,1,0]);
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
    let mut parser = Parser::new(asm);
    let mut hack = String::new();

    while parser.has_more_commands() {
        parser.advance();

        match parser.command_type() {
            CommandType::None => {
                println!("Error: Can't parse");
                return print_usage();
            },
            CommandType::A => {
                let number = i16::from_str(&parser.symbol()).unwrap();
                let mut bin = number_to_16bin(number);

                bin[0] = 0; // 先頭を0にする
                hack += &bin.iter().map(|b| b.to_string()).collect::<String>();
                hack += "\n";
            },
            CommandType::C => {
                hack += "111";
                hack += &Code::dest(&parser.dest()).unwrap();
                hack += &Code::comp(&parser.comp()).unwrap();
                hack += &Code::jump(&parser.jump()).unwrap();

                hack += "\n";
            },
            CommandType::L => { /* 今は実装しない */ },
        }
    }

    println!("{}", hack.trim());
}
