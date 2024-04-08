use std::{env, fs};

pub mod lexer;

fn main() {
    let input_file_path: String;

    let args: Vec<_> = env::args().collect();
    if args.len() > 2 {
        match args[1].as_ref() {
            "--input" => {
                input_file_path = args[2].clone();
            }
            _ => {
                panic!("Unknown option {:?}", args[1])
            }
        }
    } else {
        panic!("Usage: lexer --input ./path/to/file.gsc")
    }

    let input: String = match fs::read_to_string(input_file_path) {
        Ok(data) => data,
        Err(_) => "".to_owned(),
    };

    let lexer: lexer::Lexer = lexer::Lexer::new(input);
    lexer.tokenize();
}
