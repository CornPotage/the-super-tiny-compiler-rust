use crate::token::{Token, TokenType};
use regex::Regex;
use thiserror::Error;

pub fn tokenizer(input: String) -> anyhow::Result<Vec<Token>> {
    let whitespace: Regex = Regex::new(r"(\s|\r\n|\n|\r)").unwrap();
    let numbers: Regex = Regex::new(r"[0-9]").unwrap();
    let quotes: Regex = Regex::new(r#"(["'])"#).unwrap();
    let letters: Regex = Regex::new(r"[a-zA-Z_-]").unwrap();

    let mut current: usize = 0;
    let mut tokens: Vec<Token> = vec![];

    let input_chars: Vec<char> = input.chars().collect::<Vec<char>>();
    let input_chars_length: usize = input_chars.len();

    let get_char = |current: usize| -> char {
        return input_chars[current];
    };
    let consume_char = |current: &mut usize| -> char {
        let ch = input_chars[*current];
        *current += 1;
        return ch;
    };

    let is_eos = |current: usize| -> bool {
        return current < input_chars_length;
    };

    while is_eos(current) {
        // skip white space in a row
        let ch = get_char(current);
        if whitespace.is_match(&ch.to_string()) {
            while is_eos(current) {
                let ch = get_char(current);
                if whitespace.is_match(&ch.to_string()) {
                    consume_char(&mut current);
                } else {
                    break;
                }
            }
            continue;
        }

        // paren
        let ch = get_char(current);
        if ch == '(' || ch == ')' {
            let token = Token {
                token_type: TokenType::PAREN,
                value: ch.to_string(),
            };
            tokens.push(token);
            consume_char(&mut current);
            continue;
        }

        // number
        let ch = get_char(current);
        if numbers.is_match(&ch.to_string()) {
            let mut value: String = String::from("");
            while is_eos(current) {
                let ch = get_char(current);
                if numbers.is_match(&ch.to_string()) {
                    let ch = consume_char(&mut current);
                    value.push(ch);
                } else {
                    break;
                }
            }

            let token = Token {
                token_type: TokenType::NUMBER,
                value,
            };
            tokens.push(token);
            continue;
        }

        // string
        let ch = get_char(current);
        if quotes.is_match(&ch.to_string()) {
            consume_char(&mut current);

            let mut value: String = String::from("");
            while is_eos(current) {
                let ch = get_char(current);
                if quotes.is_match(&ch.to_string()) == false {
                    let ch = consume_char(&mut current);
                    value.push(ch);
                } else {
                    break;
                }
            }
            consume_char(&mut current);

            let token = Token {
                token_type: TokenType::STRING,
                value,
            };
            tokens.push(token);
            continue;
        }

        // letters
        let ch = get_char(current);
        if letters.is_match(&ch.to_string()) {
            let mut value: String = String::from("");
            while is_eos(current) {
                let ch = get_char(current);
                if letters.is_match(&ch.to_string()) {
                    let ch = consume_char(&mut current);
                    value.push(ch);
                } else {
                    break;
                }
            }

            let token = Token {
                token_type: TokenType::NAME,
                value,
            };
            tokens.push(token);
            continue;
        }

        return Err(TokenizeError::UnknownCharacter(ch.to_string()).into());
    }
    return Ok(tokens);
}

#[derive(Debug, Error)]
pub enum TokenizeError {
    #[error("Error: Unknown character: {0}")]
    UnknownCharacter(String)
}

#[cfg(test)]
mod tokenizer_tests {
    use crate::tokenizer::{tokenizer, TokenizeError};
    use thiserror::Error;
    use crate::token::{Token, TokenType};

    #[test]
    fn test_tokenize_valid_code() {
        let code = "
            (add 2 (subtract 4 2))
            (fullName 'hoge' 'foo')
        ";

        let tokenize_result = tokenizer(code.to_string());
        let result_tokens = tokenize_result.unwrap();

        let expected_tokens = vec![
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

        assert_eq!(result_tokens, expected_tokens);
    }

    #[test]
    fn test_tokenize_unknown_character() {
        let code = "*";

        let tokenize_result = tokenizer(code.to_string());
        let result_error = tokenize_result.unwrap_err();
        let route_cause = result_error.root_cause();

        let expected_error = TokenizeError::UnknownCharacter("*".to_string());

        assert_eq!(format!("{}", route_cause), "Error: Unknown character: *");
    }
}