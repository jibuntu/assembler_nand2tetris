// nand2tetris - 6章

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::str::FromStr;

mod parser;
use parser::Parser;
use parser::CommandType;
mod code;
use code::Code;
mod symbol_table;
use symbol_table::SymbolTable;


fn print_usage() {
    println!("Usage: command <filename> <output filename>");
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

/// symbol tableを作成する。このsymbol tableに変数シンボルは含まれない。
fn get_symbol_table(asm: &str) -> SymbolTable {
    let mut parser = Parser::new(asm.to_string());
    let mut stable = SymbolTable::new();
    let mut count = 0; // コマンド数のカウンター

    // 定義済みシンボルの設定
    stable.add_entry("SP", 0);
    stable.add_entry("LCL", 1);
    stable.add_entry("ARG", 2);
    stable.add_entry("THIS", 3);
    stable.add_entry("THAT", 4);
    stable.add_entry("R0", 0);
    stable.add_entry("R1", 1);
    stable.add_entry("R2", 2);
    stable.add_entry("R3", 3);
    stable.add_entry("R4", 4);
    stable.add_entry("R5", 5);
    stable.add_entry("R6", 6);
    stable.add_entry("R7", 7);
    stable.add_entry("R8", 8);
    stable.add_entry("R9", 9);
    stable.add_entry("R10", 10);
    stable.add_entry("R11", 11);
    stable.add_entry("R12", 12);
    stable.add_entry("R13", 13);
    stable.add_entry("R14", 14);
    stable.add_entry("R15", 15);
    stable.add_entry("SCREEN", 0x4000);
    stable.add_entry("KBD", 0x6000);

    while parser.has_more_commands() {
        parser.advance();

        match parser.command_type() {
            CommandType::A | CommandType::C => count += 1,
            CommandType::L => stable.add_entry(&parser.symbol(), count),
            CommandType::None => ()
        }
    }

    stable
}
#[test]
fn test_get_symbol_table() {
    let asm = r#"
    @10
    "#.to_string();
    assert_eq!(get_symbol_table(&asm), get_symbol_table(""));

    let asm = r#"
    (TEST)
    "#.to_string();
    let mut stable = get_symbol_table("");
    stable.add_entry("TEST", 0);
    assert_eq!(get_symbol_table(&asm), stable);

    let asm = r#"
    (TEST)
    @10
    (SYMBOL)
    @10
    "#.to_string();
    let mut stable = get_symbol_table("");
    stable.add_entry("SYMBOL", 1);
    stable.add_entry("TEST", 0);
    assert_eq!(get_symbol_table(&asm), stable);
}

/// アセンブリ言語の文字列を機械語の文字列に変換する
fn asm_to_hack(asm: String) -> Option<String> {
    let mut stable = get_symbol_table(&asm);
    let mut parser = Parser::new(asm);
    let mut hack = String::new();
    let mut vcount = 15; // 変数シンボルのカウンター
    
    // パースしてhackに代入
    while parser.has_more_commands() {
        parser.advance();

        match parser.command_type() {
            CommandType::None => return None,
            CommandType::A => {
                let mut bin = match i16::from_str(&parser.symbol()) {
                    Ok(number) => {
                        number_to_16bin(number)
                    },
                    Err(_) => {
                        let symbol = parser.symbol();
                        if stable.contains(&symbol) {
                            let address = *stable.get_address(&symbol).unwrap();
                            number_to_16bin(address as i16)
                        } else {
                            vcount+=1;
                            stable.add_entry(&symbol, vcount as usize);
                            number_to_16bin(vcount as i16)
                        }
                    }
                };

                bin[0] = 0; // 先頭を0にする
                hack += &bin.iter().map(|b| b.to_string()).collect::<String>();
                hack += "\n";
            },
            CommandType::C => {
                hack += "111";
                hack += &Code::comp(&parser.comp()).unwrap();
                hack += &Code::dest(&parser.dest()).unwrap();
                hack += &Code::jump(&parser.jump()).unwrap();

                hack += "\n";
            },
            CommandType::L => ()
        }
    }

    Some(hack)
}
#[test]
fn test_asm_to_hack() {
    let asm = r#"
    @0
    "#.to_string();
    assert_eq!(&asm_to_hack(asm).unwrap(), 
               concat!("0000", "0000", "0000", "0000", "\n"));

    let asm = r#"
    @R0
    D=M
    @R1
    "#.to_string();
    assert_eq!(&asm_to_hack(asm).unwrap(), 
               concat!("0000", "0000", "0000", "0000", "\n",
                       "1111", "1100", "0001", "0000", "\n",
                       "0000", "0000", "0000", "0001", "\n"));

    let asm = r#"
    @var
    "#.to_string();
    assert_eq!(&asm_to_hack(asm).unwrap(), 
               concat!("0000", "0000", "0001", "0000", "\n"));

    let asm = r#"
    @var
    @var
    "#.to_string();
    assert_eq!(&asm_to_hack(asm).unwrap(), 
               concat!("0000", "0000", "0001", "0000", "\n",
                       "0000", "0000", "0001", "0000", "\n"));

    let asm = r#"
    @var
    @var2
    "#.to_string();
    assert_eq!(&asm_to_hack(asm).unwrap(), 
               concat!("0000", "0000", "0001", "0000", "\n",
                       "0000", "0000", "0001", "0001", "\n"));

    let asm = r#"
    (TEST)
    @TEST
    "#.to_string();
    assert_eq!(&asm_to_hack(asm).unwrap(),
               concat!("0000", "0000", "0000", "0000", "\n"));

    let asm = r#"
    @TEST
    (TEST)
    @TENISS
    (TENISS)
    "#.to_string();
    assert_eq!(&asm_to_hack(asm).unwrap(), 
               concat!("0000", "0000", "0000", "0001", "\n",
                       "0000", "0000", "0000", "0010", "\n"));

    let asm = r#"
    @TEST
    (TEST)
    @var
    "#.to_string();
    assert_eq!(&asm_to_hack(asm).unwrap(), 
               concat!("0000", "0000", "0000", "0001", "\n",
                       "0000", "0000", "0001", "0000", "\n"));
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

    let output_file_name = match env::args().nth(2) {
        Some(file_name) => file_name,
        None => {
            println!("Error: Output file name is not exist.");
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
    let hack = match asm_to_hack(asm) {
        Some(hack) => hack,
        None => {
            println!("Error: Can't parse");
            return print_usage();
        }
    };
    
    let mut output_file = match File::create(&output_file_name) {
        Ok(file) => file,
        Err(_) => {
            println!("Error: {} is not exist.", &file_name);
            print_usage();
            return;
        }
    };

    let _ = output_file.write(hack.trim().as_bytes());
}