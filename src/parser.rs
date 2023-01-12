use std::rc::Rc;
use thiserror::Error;
use crate::token::{Token, TokenType};
use crate::ast::{ASTNode, ASTNodeType, CallExpression, NumberLiteral, StringLiteral, Program};

pub fn parser(tokens: Vec<Token>) -> anyhow::Result<Program> {
    let mut current: usize = 0;
    let tokens_length: usize = tokens.len();

    let is_eos = |current: usize| -> bool {
        return current < tokens_length;
    };

    let mut program = Program {
        node_type: ASTNodeType::Program,
        body: vec![],
    };

    while is_eos(current) {
        let ast_node = walk(&mut current, &tokens)?;
        program.body.push(Rc::new(ast_node));
    }

    return Ok(program);
}

fn walk(current: &mut usize, tokens: &Vec<Token>) -> anyhow::Result<ASTNode> {
    let get_token = |current: usize| -> &Token {
        return &tokens[current];
    };
    let consume_token = |current: &mut usize| -> &Token {
        let token = &tokens[*current];
        *current += 1;
        return token;
    };

    let token = get_token(*current);
    let token_type = token.token_type;
    let token_value = &token.value;

    if token_type == TokenType::NUMBER {
        consume_token(current);

        let ast_node = NumberLiteral {
            node_type: ASTNodeType::NumberLiteral,
            value: token_value.clone(),
        };
        return Ok(ASTNode::NumberLiteral(ast_node));
    }

    if token_type == TokenType::STRING {
        consume_token(current);

        let ast_node = StringLiteral {
            node_type: ASTNodeType::StringLiteral,
            value: token_value.clone(),
        };
        return Ok(ASTNode::StringLiteral(ast_node));
    }

    if token_type == TokenType::PAREN && token_value == "(" {
        consume_token(current);

        let parent_exp_token = get_token(*current);
        let parent_exp_token_value = &parent_exp_token.value;

        let mut ast_node = CallExpression {
            node_type: ASTNodeType::CallExpression,
            value: parent_exp_token_value.clone(),
            params: vec![],
        };

        let mut exp_token = consume_token(current);
        let mut exp_token_type = exp_token.token_type;
        let mut exp_token_value = &exp_token.value;

        loop {
            if exp_token_type == TokenType::PAREN && exp_token_value == ")" {
                break;
            }

            let child_node = walk(current, &tokens)?;
            ast_node.params.push(Rc::new(child_node));

            exp_token = get_token(*current);
            exp_token_type = exp_token.token_type;
            exp_token_value = &exp_token.value;
        }

        consume_token(current);

        return Ok(ASTNode::CallExpression(ast_node));
    }

    return Err(ParseError::UnknownToken(token_value.to_string()).into());
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Error: Unknown token: {0}")]
    UnknownToken(String)
}

#[cfg(test)]
mod parser_tests {
    use std::rc::Rc;
    use crate::ast::{ASTNodeType, Program, CallExpression, NumberLiteral, ASTNode, StringLiteral};
    use crate::parser::parser;
    use crate::token::{Token, TokenType};

    #[test]
    fn test_parse_valid_code() {
        let tokens = vec![
            Token { token_type: TokenType::PAREN, value: "(".to_string() },
            Token { token_type: TokenType::NAME, value: "add".to_string() },
            Token { token_type: TokenType::NUMBER, value: "2".to_string() },
            Token { token_type: TokenType::PAREN, value: "(".to_string() },
            Token { token_type: TokenType::NAME, value: "subtract".to_string() },
            Token { token_type: TokenType::NUMBER, value: "4".to_string() },
            Token { token_type: TokenType::NUMBER, value: "2".to_string() },
            Token { token_type: TokenType::PAREN, value: ")".to_string() },
            Token { token_type: TokenType::PAREN, value: ")".to_string() },
            Token { token_type: TokenType::PAREN, value: "(".to_string() },
            Token { token_type: TokenType::NAME, value: "fullName".to_string() },
            Token { token_type: TokenType::STRING, value: "hoge".to_string() },
            Token { token_type: TokenType::STRING, value: "foo".to_string() },
            Token { token_type: TokenType::PAREN, value: ")".to_string() },
        ];

        let parse_result = parser(tokens);
        let result_program = parse_result.unwrap();

        let expected_program = Program {
            node_type: ASTNodeType::Program,
            body: vec![
                Rc::new(ASTNode::CallExpression(CallExpression {
                    node_type: ASTNodeType::CallExpression,
                    value: "add".to_string(),
                    params: vec![
                        Rc::new(ASTNode::NumberLiteral(NumberLiteral {
                            node_type: ASTNodeType::NumberLiteral,
                            value: "2".to_string(),
                        })),
                        Rc::new(ASTNode::CallExpression(CallExpression {
                            node_type: ASTNodeType::CallExpression,
                            value: "subtract".to_string(),
                            params: vec![
                                Rc::new(ASTNode::NumberLiteral(NumberLiteral {
                                    node_type: ASTNodeType::NumberLiteral,
                                    value: "4".to_string(),
                                })),
                                Rc::new(ASTNode::NumberLiteral(NumberLiteral {
                                    node_type: ASTNodeType::NumberLiteral,
                                    value: "2".to_string(),
                                })),
                            ],
                        })),
                    ],
                })),
                Rc::new(ASTNode::CallExpression(CallExpression {
                    node_type: ASTNodeType::CallExpression,
                    value: "fullName".to_string(),
                    params: vec![
                        Rc::new(ASTNode::StringLiteral(StringLiteral {
                            node_type: ASTNodeType::StringLiteral,
                            value: "hoge".to_string(),
                        })),
                        Rc::new(ASTNode::StringLiteral(StringLiteral {
                            node_type: ASTNodeType::StringLiteral,
                            value: "foo".to_string(),
                        })),
                    ]
                }))
            ],
        };

        assert_eq!(result_program, expected_program);
    }
}
