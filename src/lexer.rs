#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
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
    // KEYWORDS
    KwThread,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub content: String,
}

impl Token {
    pub fn new(token_type: TokenType, content: String) -> Self {
        Self {
            token_type,
            content,
        }
    }

    pub fn print_token(&self) -> String {
        match self.token_type {
            TokenType::Identifier => "<Identifier>".to_owned() + self.content.as_str(),
            TokenType::String => "<String>".to_owned() + self.content.as_str(),
            TokenType::Terminator => "<Terminator>".to_owned(),
            TokenType::LParen => "<LParen>".to_owned(),
            TokenType::RParen => "<RParen>".to_owned(),
            TokenType::Comma => "<Comma>".to_owned(),
            TokenType::LCurly => "<LCurly>".to_owned(),
            TokenType::RCurly => "<RCurly>".to_owned(),
            TokenType::Plus => "<Plus>".to_owned(),
            TokenType::Assign => "<Assign>".to_owned(),
            TokenType::KwThread => "<Keyword>".to_owned() + self.content.as_str(),
        }
    }
}

pub struct Lexer {
    input: String,
    current_index: usize,
    current_buffer: String,
    pub tokens: Vec<Token>,
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
        // Check if identifier is a keyword, if true, change tokentype to that keywords type
        let content: String = self.current_buffer.clone();

        if content != "" {
            let token_type: TokenType = match content.as_str() {
                "thread" => TokenType::KwThread,
                _ => TokenType::Identifier,
            };

            self.tokens.push(Token::new(token_type, content));
            self.current_buffer = "".to_owned();
        }
    }

    pub fn tokenize(&mut self) {
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

        for token in &self.tokens {
            println!("[TOKEN] ({:?}): '{1}'", token.token_type, token.content);
        }
    }
}
