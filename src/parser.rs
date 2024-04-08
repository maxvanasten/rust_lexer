use crate::lexer;

#[derive(Debug)]
struct Node {
    identifier: String,
    data: String,
    children: Vec<Node>,
}

#[derive(Debug)]
pub enum RuleType {
    VarAssignString,
    VarAssignNum,
}

pub struct Rule {
    identifier: RuleType,
    sequence: Vec<lexer::TokenType>,
}

impl Rule {
    pub fn new(identifier: RuleType, sequence: Vec<lexer::TokenType>) -> Self {
        Self {
            identifier,
            sequence,
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

        let mut nodes: Vec<Node> = vec![];

        while !parsing_done {
            let mut current_node: Node = Node {
                identifier: "UNKNOWN".to_owned(),
                data: "".to_owned(),
                children: vec![],
            };

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

                        // NOTE: THIS IS WHERE RULES GET TURNED INTO AST NODES
                        match rule.identifier {
                            RuleType::VarAssignString => {
                                current_node.identifier = "VarAssignString".to_owned();
                                current_node.children = vec![
                                    Node {
                                        identifier: "name".to_owned(),
                                        data: rule_tokens[0].content.clone(),
                                        children: vec![],
                                    },
                                    Node {
                                        identifier: "value".to_owned(),
                                        data: rule_tokens[2].content.clone(),
                                        children: vec![],
                                    },
                                ]
                            }
                            RuleType::VarAssignNum => {
                                current_node.identifier = "VarAssignNum".to_owned();
                                current_node.children = vec![
                                    Node {
                                        identifier: "name".to_owned(),
                                        data: rule_tokens[0].content.clone(),
                                        children: vec![],
                                    },
                                    Node {
                                        identifier: "value".to_owned(),
                                        data: rule_tokens[2].content.clone(),
                                        children: vec![],
                                    },
                                ]
                            }
                        }

                        nodes.push(current_node);
                        current_node = Node {
                            identifier: "".to_owned(),
                            data: "".to_owned(),
                            children: vec![],
                        };

                        println!("Found matching rule: {:?} ({1})", rule.identifier, rule_str)
                    }
                }
            }
        }

        // Parsing is done
        for node in nodes {
            println!(
                "[NODE] Id: {0}, data: {1}, children: {2:?}",
                node.identifier, node.data, node.children
            );
        }
    }
}
