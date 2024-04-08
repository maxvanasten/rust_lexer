use std::{env, fs};

#[derive(Debug)]
enum TokenType {
    Identifier, // ANY
    String,     // "ANY"
    Terminator, // ;
    LParen,     // (
    RParen,     // )
    Comma,      // ,
    LCurly,     // {
    RCurly,     // }
    Plus,       // +
    Assign,     // =
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    content: String,
}

impl Token {
    pub fn new(token_type: TokenType, content: String) -> Self {
        Self {
            token_type,
            content,
        }
    }
}

struct Lexer {
    input: String,
    current_index: usize,
    current_buffer: String,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input,
            current_index: 0,
            current_buffer: "".to_owned(),
            tokens: vec![],
        }
    }

    fn current_character(&self) -> char {
        match self.input.chars().nth(self.current_index) {
            Some(character) => character,
            None => todo!(),
        }
    }

    fn push_identifier(&mut self) {
        self.tokens.push(Token::new(
            TokenType::Identifier,
            self.current_buffer.clone(),
        ));
        self.current_buffer = "".to_owned();
    }

    pub fn parse_input(mut self) {
        while self.input.len() > self.current_index {
            let character = self.current_character();
            match character {
                '=' => {
                    self.push_identifier();

                    self.tokens
                        .push(Token::new(TokenType::Assign, "=".to_owned()));
                    self.current_index += 1;
                }
                ';' => {
                    self.push_identifier();

                    self.tokens
                        .push(Token::new(TokenType::Terminator, ";".to_owned()));

                    self.current_index += 1;
                }
                '"' => {
                    self.push_identifier();

                    self.current_index += 1;

                    while self.current_character() != '"' {
                        self.current_buffer.push(self.current_character());
                        self.current_index += 1;
                    }

                    self.current_index += 1;

                    self.tokens
                        .push(Token::new(TokenType::String, self.current_buffer.clone()));
                    self.current_buffer = "".to_owned();
                }
                ' ' => {
                    self.push_identifier();

                    self.current_index += 1;
                }
                '{' => {
                    self.push_identifier();

                    self.current_index += 1;

                    self.tokens
                        .push(Token::new(TokenType::LCurly, "{".to_owned()));
                }
                '}' => {
                    self.push_identifier();

                    self.current_index += 1;

                    self.tokens
                        .push(Token::new(TokenType::RCurly, "}".to_owned()));
                }
                '(' => {
                    self.push_identifier();

                    self.current_index += 1;

                    self.tokens
                        .push(Token::new(TokenType::LParen, "(".to_owned()));
                }
                ')' => {
                    self.push_identifier();

                    self.current_index += 1;

                    self.tokens
                        .push(Token::new(TokenType::RParen, ")".to_owned()));
                }
                ',' => {
                    self.push_identifier();

                    self.current_index += 1;

                    self.tokens
                        .push(Token::new(TokenType::Comma, ",".to_owned()));
                }
                '+' => {
                    self.push_identifier();

                    self.current_index += 1;

                    self.tokens
                        .push(Token::new(TokenType::Plus, "+".to_owned()));
                }
                _ => {
                    // Skip certain characters
                    if character != '\n' && character != '\t' && character != ' ' {
                        self.current_buffer.push(character);
                    }

                    self.current_index += 1;
                }
            }
        }

        // Purge empty identifiers
        let mut parsed_tokens: Vec<Token> = vec![];
        for token in self.tokens {
            if token.content != "" {
                parsed_tokens.push(token);
            }
        }
        // Finished parsing
        for token in parsed_tokens {
            println!("Token: {:?}", token)
        }
    }
}

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

    let mut lexer: Lexer = Lexer::new(input);
    lexer.parse_input();
}
