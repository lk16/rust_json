// TODO remove when done
#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt;

#[derive(Debug)]
enum TokenType {
    Null,
    True,
    False,
    String,
    Comma,
    Colon,
    ObjectStart,
    ObjectEnd,
    ArrayStart,
    ArrayEnd,
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

    fn tokenize(mut self) -> Result<Vec<Token<'a>>, usize> {
        loop {
            let c = self.input.chars().nth(self.offset);

            let token_result = match c {
                None => break,
                Some('n') => self.tokenize_null(),
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

    fn tokenize_null(&self) -> Result<Token<'a>, usize> {
        if self.input.starts_with("null") {
            let token = Token::new(TokenType::Null, "null");
            return Ok(token);
        }
        Err(self.offset)
    }
}

fn tokenize(input: &str) -> Result<Vec<Token>, usize> {
    Tokenizer::new(input).tokenize()
}

fn main() {
    let tokenized = tokenize("null");

    match tokenized {
        Ok(tokens) => {
            println!("Got {} token(s)", tokens.len());
            for token in tokens.iter() {
                println!("- {}", token);
            }
        }
        Err(offset) => println!("Parse Error at offset {}", offset),
    }
}
