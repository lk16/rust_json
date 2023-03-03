// TODO remove when done
#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt;

#[derive(Debug, PartialEq)]
enum TokenType {
    ArrayEnd,
    ArrayStart,
    Colon,
    Comma,
    False,
    Integer,
    Null,
    ObjectEnd,
    ObjectStart,
    String,
    True,
    Whitespace,
}

#[derive(Debug, PartialEq)]
struct TokenizeError {
    offset: usize,
    message: &'static str,
}

impl TokenizeError {
    fn new(offset: usize, message: &'static str) -> Self {
        Self { offset, message }
    }
}

#[derive(Debug, PartialEq)]
struct Token<'a> {
    type_: TokenType,
    value: &'a str,
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "type={:?} value={}", self.type_, self.value)
    }
}

impl<'a> Token<'a> {
    fn new(type_: TokenType, value: &'a str) -> Self {
        Self { type_, value }
    }

    fn len(&self) -> usize {
        self.value.len()
    }
}

struct Tokenizer<'a> {
    input: &'a str,
    offset: usize,
    tokens: Vec<Token<'a>>,
}

impl<'a> Tokenizer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input,
            offset: 0,
            tokens: vec![],
        }
    }

    fn tokenize(mut self) -> Result<Vec<Token<'a>>, TokenizeError> {
        loop {
            let c = self.input.chars().nth(self.offset);

            let token_result = match c {
                None => break,
                Some(',') => self.tokenize_literal(",", TokenType::Comma),
                Some(':') => self.tokenize_literal(":", TokenType::Colon),
                Some('[') => self.tokenize_literal("[", TokenType::ArrayStart),
                Some(']') => self.tokenize_literal("]", TokenType::ArrayEnd),
                Some('{') => self.tokenize_literal("{", TokenType::ObjectStart),
                Some('}') => self.tokenize_literal("}", TokenType::ObjectEnd),
                Some('f') => self.tokenize_literal("false", TokenType::False),
                Some('n') => self.tokenize_literal("null", TokenType::Null),
                Some('t') => self.tokenize_literal("true", TokenType::True),
                Some('"') => self.tokenize_string(),
                Some(c) => {
                    if c.is_ascii_digit() {
                        self.tokenize_integer()
                    } else {
                        if c.is_ascii_whitespace() {
                            self.tokenize_whitespace()
                        } else {
                            Err(TokenizeError::new(self.offset, "Unhandled character"))
                        }
                    }
                }
            };

            match token_result {
                Ok(token) => {
                    self.offset += token.len();
                    self.tokens.push(token);
                }
                Err(offset) => return Err(offset),
            }
        }

        Ok(self.tokens)
    }

    fn tokenize_literal(
        &self,
        literal: &'static str,
        type_: TokenType,
    ) -> Result<Token<'a>, TokenizeError> {
        if self.input.split_at(self.offset).1.starts_with(literal) {
            let token = Token::new(type_, literal);
            return Ok(token);
        }
        // TODO make error message better
        Err(TokenizeError::new(self.offset, "expected literal"))
    }

    fn tokenize_integer(&self) -> Result<Token<'a>, TokenizeError> {
        let mut int_end_offset = self.offset;
        let chars = self.input.chars().skip(self.offset);

        for c in chars {
            if c.is_ascii_digit() {
                int_end_offset += 1;
            } else {
                break;
            }
        }

        let token = Token::new(TokenType::Integer, &self.input[self.offset..int_end_offset]);
        Ok(token)
    }

    fn tokenize_string(&self) -> Result<Token<'a>, TokenizeError> {
        let quote_distance = self
            .input
            .chars()
            .skip(self.offset + 1)
            .position(|x| x == '"');

        match quote_distance {
            None => Err(TokenizeError::new(
                self.offset,
                "No string-terminating quote found",
            )),
            Some(quote_distance) => {
                let str_end_offset = self.offset + quote_distance + 2;
                let value = &self.input[self.offset..str_end_offset];
                let token = Token::new(TokenType::String, value);
                Ok(token)
            }
        }
    }

    fn tokenize_whitespace(&self) -> Result<Token<'a>, TokenizeError> {
        let mut ws_end_offset = self.offset;
        let chars = self.input.chars().skip(self.offset);

        for c in chars {
            if c.is_ascii_whitespace() {
                ws_end_offset += 1;
            } else {
                break;
            }
        }

        let token = Token::new(
            TokenType::Whitespace,
            &self.input[self.offset..ws_end_offset],
        );
        Ok(token)
    }
}

fn tokenize(input: &str) -> Result<Vec<Token>, TokenizeError> {
    Tokenizer::new(input).tokenize()
}

fn main() {
    let tokenized = tokenize("\"foo\" 123 , [] {} \"hello world\" false null truetrue");

    match tokenized {
        Ok(tokens) => {
            println!("Got {} token(s)", tokens.len());
            for token in tokens.iter() {
                println!("- {}", token);
            }
        }
        Err(error) => println!("Parse Error at offset {}: {}", error.offset, error.message),
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenize;
    use crate::Token;
    use crate::TokenType;
    use crate::TokenizeError;

    #[test]
    fn test_tokenize() {
        let cases: Vec<(&str, Result<Vec<Token>, TokenizeError>)> = vec![
            ("null", Ok(vec![Token::new(TokenType::Null, "null")])),
            ("true", Ok(vec![Token::new(TokenType::True, "true")])),
            ("false", Ok(vec![Token::new(TokenType::False, "false")])),
            (":", Ok(vec![Token::new(TokenType::Colon, ":")])),
            ("[", Ok(vec![Token::new(TokenType::ArrayStart, "[")])),
            ("]", Ok(vec![Token::new(TokenType::ArrayEnd, "]")])),
            ("{", Ok(vec![Token::new(TokenType::ObjectStart, "{")])),
            ("}", Ok(vec![Token::new(TokenType::ObjectEnd, "}")])),
            (",", Ok(vec![Token::new(TokenType::Comma, ",")])),
            ("1234", Ok(vec![Token::new(TokenType::Integer, "1234")])),
            (
                " \n\r ",
                Ok(vec![Token::new(TokenType::Whitespace, " \n\r ")]),
            ),
            (
                "\"Hello world\"",
                Ok(vec![Token::new(TokenType::String, "\"Hello world\"")]),
            ),
            (
                "broken",
                Err(TokenizeError {
                    offset: 0,
                    message: "Unhandled character",
                }),
            ),
            (
                "\"no closing quote",
                Err(TokenizeError {
                    offset: 0,
                    message: "No string-terminating quote found",
                }),
            ),
        ];

        for case in cases.iter() {
            assert_eq!(tokenize(case.0), case.1)
        }
    }
}
