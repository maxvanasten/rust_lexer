use std::{env, fs};

mod lexer;
mod parser;

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

    let mut lexer: lexer::Lexer = lexer::Lexer::new(input);
    // Turn input file into a list of tokens
    lexer.tokenize();

    let parser: parser::Parser = parser::Parser::new(vec![
        parser::Rule::new(
            "StringVarAssign".to_owned(),
            vec![
                lexer::TokenType::Identifier,
                lexer::TokenType::Assign,
                lexer::TokenType::String,
                lexer::TokenType::Terminator,
            ],
        ),
        parser::Rule::new(
            "NumVarAssign".to_owned(),
            vec![
                lexer::TokenType::Identifier,
                lexer::TokenType::Assign,
                lexer::TokenType::Identifier,
                lexer::TokenType::Terminator,
            ],
        ),
    ]);

    parser.parse(lexer.tokens);
}
