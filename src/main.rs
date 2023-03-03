// TODO remove when done
#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt;

#[derive(Debug)]
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
}

struct TokenizeError {
    offset: usize,
    message: &'static str,
}

impl TokenizeError {
    fn new(offset: usize, message: &'static str) -> Self {
        Self { offset, message }
    }
}

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
                Some(_) => todo!(),
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
}

fn tokenize(input: &str) -> Result<Vec<Token>, TokenizeError> {
    Tokenizer::new(input).tokenize()
}

fn main() {
    let tokenized = tokenize("null,,:[]{}falsetruefalsefalse");

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
