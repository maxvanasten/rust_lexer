use crate::lexer;

pub struct Rule {
    name: String,
    sequence: Vec<lexer::TokenType>,
    tokens: Vec<lexer::Token>,
}

impl Rule {
    pub fn new(name: String, sequence: Vec<lexer::TokenType>) -> Self {
        Self {
            name,
            sequence,
            tokens: vec![],
        }
    }
}

pub struct Parser {
    rules: Vec<Rule>,
}

impl Parser {
    pub fn new(rules: Vec<Rule>) -> Self {
        Self { rules }
    }

    pub fn parse(&self, tokens: Vec<lexer::Token>) {
        // Loop through tokens
        // check if current sequence conforms to a rule
        let mut parsing_done: bool = false;
        let mut absolute_token_index: usize = 0;
        while !parsing_done {
            for rule in &self.rules {
                // For every rule, loop through the tokens at the current index and see if they fit
                let mut done: bool = false;
                let mut valid: bool = true;
                let mut relative_token_index: usize = 0;
                let mut rule_tokens: Vec<&lexer::Token> = vec![];
                while valid && !done {
                    let token = &tokens[absolute_token_index + relative_token_index];
                    let sequence_type = &rule.sequence[relative_token_index];

                    if token.token_type == *sequence_type {
                        rule_tokens.push(token);
                        relative_token_index += 1;
                    } else {
                        rule_tokens = vec![];
                        valid = false;
                    }

                    if relative_token_index == rule.sequence.len() {
                        done = true;
                        absolute_token_index += relative_token_index;
                        if absolute_token_index >= tokens.len() {
                            parsing_done = true;
                        }
                        let mut rule_str: String = "".to_owned();

                        for tok in &rule_tokens {
                            rule_str += lexer::Token::print_token(tok).as_str();
                        }

                        println!("Found matching rule: {0} ({1})", rule.name, rule_str)
                    }
                }
            }
        }
    }
}
