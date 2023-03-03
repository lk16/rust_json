use std::collections::HashMap;
use std::{env, fmt};

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
    message: String,
}

impl TokenizeError {
    fn new(offset: usize, message: String) -> Self {
        Self { offset, message }
    }
}

#[derive(Debug, PartialEq)]
struct Token {
    type_: TokenType,
    value: String,
    offset: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "type={:?} value={}", self.type_, self.value)
    }
}

impl Token {
    fn new(type_: TokenType, value: &str, offset: usize) -> Self {
        Self {
            type_,
            value: value.to_owned(),
            offset,
        }
    }

    fn len(&self) -> usize {
        self.value.len()
    }
}

struct Tokenizer {
    input: String,
    offset: usize,
    tokens: Vec<Token>,
}

impl Tokenizer {
    fn new(input: &str) -> Self {
        Self {
            input: input.to_owned(),
            offset: 0,
            tokens: vec![],
        }
    }

    fn tokenize(mut self) -> Result<Vec<Token>, TokenizeError> {
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
                            Err(TokenizeError::new(
                                self.offset,
                                "Unhandled character".to_owned(),
                            ))
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
    ) -> Result<Token, TokenizeError> {
        if self.input.split_at(self.offset).1.starts_with(literal) {
            let token = Token::new(type_, literal, self.offset);
            return Ok(token);
        }
        // TODO make error message better
        Err(TokenizeError::new(
            self.offset,
            format!("Expected literal `{}`", literal),
        ))
    }

    fn tokenize_integer(&self) -> Result<Token, TokenizeError> {
        let mut int_end_offset = self.offset;
        let chars = self.input.chars().skip(self.offset);

        for c in chars {
            if c.is_ascii_digit() {
                int_end_offset += 1;
            } else {
                break;
            }
        }

        let token = Token::new(
            TokenType::Integer,
            &self.input[self.offset..int_end_offset],
            self.offset,
        );
        Ok(token)
    }

    fn tokenize_string(&self) -> Result<Token, TokenizeError> {
        let quote_distance = self
            .input
            .chars()
            .skip(self.offset + 1)
            .position(|x| x == '"');

        match quote_distance {
            None => Err(TokenizeError::new(
                self.offset,
                "No string-terminating quote found".to_owned(),
            )),
            Some(quote_distance) => {
                let str_end_offset = self.offset + quote_distance + 2;
                let value = &self.input[self.offset..str_end_offset];
                let token = Token::new(TokenType::String, value, self.offset);
                Ok(token)
            }
        }
    }

    fn tokenize_whitespace(&self) -> Result<Token, TokenizeError> {
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
            self.offset,
        );
        Ok(token)
    }
}

fn tokenize(input: &str) -> Result<Vec<Token>, TokenizeError> {
    Tokenizer::new(input).tokenize()
}

#[derive(Debug, PartialEq)]
enum Json {
    Null,
    Boolean(bool),
    Integer(i32),
    String(String),
    Array(Vec<Json>),
    Object(HashMap<String, Json>),
}

struct Parser {
    tokens: Vec<Token>,
    offset: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>, offset: usize) -> Self {
        Self { tokens, offset }
    }

    fn parse(&mut self) -> Json {
        self.tokens.retain(|x| x.type_ != TokenType::Whitespace);
        let parsed = self._parse();

        if self.offset != self.tokens.len() {
            todo!()
        }

        parsed
    }

    fn parse_string(&mut self) -> Json {
        let s = self.parse_object_key();
        Json::String(s)
    }

    fn parse_object_key(&mut self) -> String {
        let token = &self.tokens.get(self.offset);

        match token {
            None => todo!(),
            Some(token) => {
                if token.type_ != TokenType::String {
                    todo!()
                }

                self.offset += 1;
                let s = token.value[1..token.value.len() - 1].to_owned();
                s
            }
        }
    }

    fn parse_array(&mut self) -> Json {
        let mut array: Vec<Json> = vec![];

        // Consume `[` chracter
        self.offset += 1;

        let token = &self.tokens.get(self.offset);
        match token {
            None => todo!(),
            Some(token) => {
                if token.type_ == TokenType::ArrayEnd {
                    // Found empty array
                    self.offset += 1;
                    return Json::Array(array);
                }
            }
        }

        loop {
            let item = self._parse();
            array.push(item);

            let token = &self.tokens.get(self.offset);

            match token {
                None => todo!(),
                Some(token) => match token.type_ {
                    TokenType::Comma => self.offset += 1,
                    TokenType::ArrayEnd => {
                        self.offset += 1;
                        break;
                    }
                    _ => todo!(),
                },
            }
        }

        Json::Array(array)
    }

    fn parse_object(&mut self) -> Json {
        let mut object: HashMap<String, Json> = HashMap::new();

        // Consume `{` chracter
        self.offset += 1;

        let token = &self.tokens.get(self.offset);

        match token {
            None => todo!(),
            Some(token) => {
                if token.type_ == TokenType::ObjectEnd {
                    // Found empty object
                    self.offset += 1;
                    return Json::Object(object);
                }
            }
        }

        loop {
            let key = self.parse_object_key();

            let token = &self.tokens.get(self.offset);

            match token {
                None => todo!(),
                Some(token) => match token.type_ {
                    TokenType::Colon => {
                        self.offset += 1;

                        let value = self._parse();
                        object.insert(key, value);
                    }
                    _ => todo!(),
                },
            }

            let token = &self.tokens.get(self.offset);

            match token {
                None => todo!(),
                Some(token) => match token.type_ {
                    TokenType::Comma => self.offset += 1,
                    TokenType::ObjectEnd => {
                        self.offset += 1;
                        break;
                    }
                    _ => todo!(),
                },
            }
        }

        Json::Object(object)
    }

    fn _parse(&mut self) -> Json {
        let token = &self.tokens.get(self.offset);

        match token {
            None => todo!(),
            Some(token) => match token.type_ {
                TokenType::Null => {
                    self.offset += 1;
                    Json::Null
                }
                TokenType::True => {
                    self.offset += 1;
                    Json::Boolean(true)
                }
                TokenType::False => {
                    self.offset += 1;
                    Json::Boolean(false)
                }
                TokenType::Integer => match token.value.parse::<i32>() {
                    Ok(i) => {
                        self.offset += 1;
                        Json::Integer(i)
                    }
                    Err(_) => todo!(),
                },
                TokenType::String => self.parse_string(),
                TokenType::ArrayStart => self.parse_array(),
                TokenType::ObjectStart => self.parse_object(),
                _ => todo!(),
            },
        }
    }
}

fn parse(tokens: Vec<Token>) -> Json {
    Parser::new(tokens, 0).parse()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <json string>", args[0]);
        std::process::exit(1);
    }

    let tokenized = tokenize(&args[1]);

    match tokenized {
        Ok(tokens) => {
            let json = parse(tokens);
            println!("{:?}", json);
        }
        Err(error) => println!(
            "Tokenize Error at offset {}: {}",
            error.offset, error.message
        ),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::parse;
    use crate::tokenize;
    use crate::Json;
    use crate::Token;
    use crate::TokenType;
    use crate::TokenizeError;

    #[test]
    fn test_tokenize() {
        let cases: Vec<(&str, Result<Vec<Token>, TokenizeError>)> = vec![
            ("null", Ok(vec![Token::new(TokenType::Null, "null", 0)])),
            ("true", Ok(vec![Token::new(TokenType::True, "true", 0)])),
            ("false", Ok(vec![Token::new(TokenType::False, "false", 0)])),
            (":", Ok(vec![Token::new(TokenType::Colon, ":", 0)])),
            ("[", Ok(vec![Token::new(TokenType::ArrayStart, "[", 0)])),
            ("]", Ok(vec![Token::new(TokenType::ArrayEnd, "]", 0)])),
            ("{", Ok(vec![Token::new(TokenType::ObjectStart, "{", 0)])),
            ("}", Ok(vec![Token::new(TokenType::ObjectEnd, "}", 0)])),
            (",", Ok(vec![Token::new(TokenType::Comma, ",", 0)])),
            ("1234", Ok(vec![Token::new(TokenType::Integer, "1234", 0)])),
            (
                " \n\r ",
                Ok(vec![Token::new(TokenType::Whitespace, " \n\r ", 0)]),
            ),
            (
                "\"Hello world\"",
                Ok(vec![Token::new(TokenType::String, "\"Hello world\"", 0)]),
            ),
            (
                "123 {} [] , : \"a b\" null\nfalsetrue",
                Ok(vec![
                    Token::new(TokenType::Integer, "123", 0),
                    Token::new(TokenType::Whitespace, " ", 3),
                    Token::new(TokenType::ObjectStart, "{", 4),
                    Token::new(TokenType::ObjectEnd, "}", 5),
                    Token::new(TokenType::Whitespace, " ", 6),
                    Token::new(TokenType::ArrayStart, "[", 7),
                    Token::new(TokenType::ArrayEnd, "]", 8),
                    Token::new(TokenType::Whitespace, " ", 9),
                    Token::new(TokenType::Comma, ",", 10),
                    Token::new(TokenType::Whitespace, " ", 11),
                    Token::new(TokenType::Colon, ":", 12),
                    Token::new(TokenType::Whitespace, " ", 13),
                    Token::new(TokenType::String, "\"a b\"", 14),
                    Token::new(TokenType::Whitespace, " ", 19),
                    Token::new(TokenType::Null, "null", 20),
                    Token::new(TokenType::Whitespace, "\n", 24),
                    Token::new(TokenType::False, "false", 25),
                    Token::new(TokenType::True, "true", 30),
                ]),
            ),
            (
                "broken",
                Err(TokenizeError {
                    offset: 0,
                    message: "Unhandled character".to_owned(),
                }),
            ),
            (
                "\"no closing quote",
                Err(TokenizeError {
                    offset: 0,
                    message: "No string-terminating quote found".to_owned(),
                }),
            ),
            (
                "foo",
                Err(TokenizeError {
                    offset: 0,
                    message: "Expected literal `false`".to_owned(),
                }),
            ),
        ];

        for case in cases.iter() {
            assert_eq!(tokenize(case.0), case.1)
        }
    }

    #[test]
    fn test_parse() {
        let cases: Vec<(&str, Json)> = vec![
            ("null", Json::Null),
            ("true", Json::Boolean(true)),
            ("false", Json::Boolean(false)),
            ("[]", Json::Array(vec![])),
            ("\"hello world\"", Json::String("hello world".to_owned())),
            ("[false]", Json::Array(vec![Json::Boolean(false)])),
            ("[null]", Json::Array(vec![Json::Null])),
            (
                "[1,2,3,false]",
                Json::Array(vec![
                    Json::Integer(1),
                    Json::Integer(2),
                    Json::Integer(3),
                    Json::Boolean(false),
                ]),
            ),
            (
                "[[1],null]",
                Json::Array(vec![Json::Array(vec![Json::Integer(1)]), Json::Null]),
            ),
            ("{}", Json::Object(HashMap::new())),
            (
                "{\"foo\": 1337}",
                Json::Object(HashMap::from([("foo".to_owned(), Json::Integer(1337))])),
            ),
            (
                "{\"foo\": 1337, \"bar\": [69]}",
                Json::Object(HashMap::from([
                    ("foo".to_owned(), Json::Integer(1337)),
                    ("bar".to_owned(), Json::Array(vec![Json::Integer(69)])),
                ])),
            ),
        ];

        for case in cases.iter() {
            let tokens = tokenize(case.0).unwrap();
            let json = parse(tokens);
            assert_eq!(json, case.1);
        }
    }
}
