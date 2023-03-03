use std::collections::HashMap;

use crate::tokenizer::{Token, TokenType};

#[derive(Debug, PartialEq)]
pub enum Json {
    Null,
    Boolean(bool),
    Integer(i32),
    String(String),
    Array(Vec<Json>),
    Object(HashMap<String, Json>),
}

#[derive(Debug, PartialEq)]
pub struct ParseError {
    offset: usize,
    message: String,
}

impl ParseError {
    fn new(offset: usize, message: String) -> Self {
        Self { offset, message }
    }
}

struct Parser {
    tokens: Vec<Token>,
    offset: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>, offset: usize) -> Self {
        Self { tokens, offset }
    }

    fn parse(&mut self) -> Result<Json, ParseError> {
        self.tokens.retain(|x| x.type_ != TokenType::Whitespace);
        let parsed = self._parse()?;

        if self.offset != self.tokens.len() {
            let len = match self.tokens.last() {
                None => 0,
                Some(token) => token.offset + token.len(),
            };
            return Err(ParseError::new(
                len,
                "Unexpected extra input found".to_owned(),
            ));
        }

        Ok(parsed)
    }

    fn parse_string(&mut self) -> Result<Json, ParseError> {
        let token = &self.tokens.get(self.offset);

        match token {
            None => Err(ParseError::new(
                self.offset,
                "Unexpected end of input".to_owned(),
            )),
            Some(token) => {
                if token.type_ != TokenType::String {
                    return Err(ParseError::new(
                        self.offset,
                        format!("Cannot parse `{}` as string", token.value),
                    ));
                }

                self.offset += 1;
                let s = token.value[1..token.value.len() - 1].to_owned();
                Ok(Json::String(s))
            }
        }
    }

    fn parse_string_key(&mut self) -> Result<String, ParseError> {
        let json = self.parse_string()?;
        match json {
            Json::String(s) => Ok(s),
            _ => Err(ParseError::new(
                self.offset - 1,
                format!(
                    "Cannot use `{}` as object key",
                    self.tokens[self.offset - 1].value
                ),
            )),
        }
    }

    fn parse_array(&mut self) -> Result<Json, ParseError> {
        let mut array: Vec<Json> = vec![];

        // Consume `[` chracter
        self.offset += 1;

        let token = &self.tokens.get(self.offset);
        match token {
            None => {
                return Err(ParseError::new(
                    self.offset,
                    "Unexpected end of input".to_owned(),
                ))
            }
            Some(token) => {
                if token.type_ == TokenType::ArrayEnd {
                    // Found empty array
                    self.offset += 1;
                    return Ok(Json::Array(array));
                }
            }
        }

        loop {
            let item = self._parse()?;
            array.push(item);

            let token = &self.tokens.get(self.offset);

            match token {
                None => {
                    return Err(ParseError::new(
                        self.offset,
                        "Unexpected end of input".to_owned(),
                    ))
                }
                Some(token) => match token.type_ {
                    TokenType::Comma => self.offset += 1,
                    TokenType::ArrayEnd => {
                        self.offset += 1;
                        break;
                    }
                    _ => {
                        return Err(ParseError::new(
                            self.offset,
                            format!("Unexpected token `{}` in array", token).to_owned(),
                        ))
                    }
                },
            }
        }

        Ok(Json::Array(array))
    }

    fn parse_object(&mut self) -> Result<Json, ParseError> {
        let mut object: HashMap<String, Json> = HashMap::new();

        // Consume `{` chracter
        self.offset += 1;

        let token = &self.tokens.get(self.offset);

        match token {
            None => {
                return Err(ParseError::new(
                    self.offset,
                    "Unexpected end of input".to_owned(),
                ))
            }
            Some(token) => {
                if token.type_ == TokenType::ObjectEnd {
                    // Found empty object
                    self.offset += 1;
                    return Ok(Json::Object(object));
                }
            }
        }

        loop {
            let key = self.parse_string_key()?;

            let token = &self.tokens.get(self.offset);

            match token {
                None => {
                    return Err(ParseError::new(
                        self.offset,
                        "Unexpected end of input".to_owned(),
                    ))
                }
                Some(token) => match token.type_ {
                    TokenType::Colon => {
                        self.offset += 1;

                        let value = self._parse()?;
                        object.insert(key, value);
                    }
                    _ => {
                        return Err(ParseError::new(
                            self.offset,
                            format!("Unexpected token `{}` in object", token).to_owned(),
                        ))
                    }
                },
            }

            let token = &self.tokens.get(self.offset);

            match token {
                None => {
                    return Err(ParseError::new(
                        self.offset,
                        "Unexpected end of input".to_owned(),
                    ))
                }
                Some(token) => match token.type_ {
                    TokenType::Comma => self.offset += 1,
                    TokenType::ObjectEnd => {
                        self.offset += 1;
                        break;
                    }
                    _ => {
                        return Err(ParseError::new(
                            self.offset,
                            format!("Unexpected token `{}` in object", token).to_owned(),
                        ))
                    }
                },
            }
        }

        Ok(Json::Object(object))
    }

    fn _parse(&mut self) -> Result<Json, ParseError> {
        let token = &self.tokens.get(self.offset);

        match token {
            None => {
                return Err(ParseError::new(
                    self.offset,
                    "Unexpected end of input".to_owned(),
                ))
            }
            Some(token) => match token.type_ {
                TokenType::Null => {
                    self.offset += 1;
                    Ok(Json::Null)
                }
                TokenType::True => {
                    self.offset += 1;
                    Ok(Json::Boolean(true))
                }
                TokenType::False => {
                    self.offset += 1;
                    Ok(Json::Boolean(false))
                }
                TokenType::Integer => match token.value.parse::<i32>() {
                    Ok(i) => {
                        self.offset += 1;
                        Ok(Json::Integer(i))
                    }
                    Err(_) => Err(ParseError::new(
                        self.offset,
                        format!("Cannot parse `{}` as integer", token.value),
                    )),
                },
                TokenType::String => self.parse_string(),
                TokenType::ArrayStart => self.parse_array(),
                TokenType::ObjectStart => self.parse_object(),
                _ => Err(ParseError::new(
                    self.offset,
                    format!("Found unexpected token `{}`", token.value),
                )),
            },
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Json, ParseError> {
    Parser::new(tokens, 0).parse()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::parser::{parse, Json, ParseError};
    use crate::tokenizer::tokenize;

    #[test]
    fn test_parse() {
        let cases: Vec<(&str, Result<Json, ParseError>)> = vec![
            ("null", Ok(Json::Null)),
            ("true", Ok(Json::Boolean(true))),
            ("false", Ok(Json::Boolean(false))),
            ("[]", Ok(Json::Array(vec![]))),
            (
                "\"hello world\"",
                Ok(Json::String("hello world".to_owned())),
            ),
            ("[false]", Ok(Json::Array(vec![Json::Boolean(false)]))),
            ("[null]", Ok(Json::Array(vec![Json::Null]))),
            (
                "[1,2,3,false]",
                Ok(Json::Array(vec![
                    Json::Integer(1),
                    Json::Integer(2),
                    Json::Integer(3),
                    Json::Boolean(false),
                ])),
            ),
            (
                "[[1],null]",
                Ok(Json::Array(vec![
                    Json::Array(vec![Json::Integer(1)]),
                    Json::Null,
                ])),
            ),
            ("{}", Ok(Json::Object(HashMap::new()))),
            (
                "{\"foo\": 1337}",
                Ok(Json::Object(HashMap::from([(
                    "foo".to_owned(),
                    Json::Integer(1337),
                )]))),
            ),
            (
                "{\"foo\": 1337, \"bar\": [69]}",
                Ok(Json::Object(HashMap::from([
                    ("foo".to_owned(), Json::Integer(1337)),
                    ("bar".to_owned(), Json::Array(vec![Json::Integer(69)])),
                ]))),
            ),
        ];

        for case in cases.iter() {
            let tokens = tokenize(case.0).unwrap();
            let json = parse(tokens);
            assert_eq!(json, case.1);
        }
    }
}
