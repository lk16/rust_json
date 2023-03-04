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
    pub offset: usize,
    pub message: String,
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

        if self.offset < self.tokens.len() {
            return Err(ParseError::new(
                self.offset,
                "Unexpected extra input found".to_owned(),
            ));
        }

        Ok(parsed)
    }

    fn parse_string(&mut self) -> Result<Json, ParseError> {
        let s = self.parse_string_key()?;
        Ok(Json::String(s))
    }

    fn parse_string_key(&mut self) -> Result<String, ParseError> {
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
                Ok(s)
            }
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
                            format!("Unexpected token `{}` in array", token.value),
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
                            format!("Unexpected token `{}` in object", token.value),
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
                            format!("Unexpected token `{}` in object", token.value),
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
            None => Err(ParseError::new(
                self.offset,
                "Unexpected end of input".to_owned(),
            )),
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

    macro_rules! parser_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                let tokens = tokenize(input).unwrap();
                let json = parse(tokens);
                assert_eq!(json, expected);

            }
        )*
        }
    }

    parser_tests! {
        test_parse_null: ("null", Ok(Json::Null)),
        test_parse_true: ("true", Ok(Json::Boolean(true))),
        test_parse_false:    ("false", Ok(Json::Boolean(false))),
        test_parse_empty_list: ("[]", Ok(Json::Array(vec![]))),
        test_parse_string: (
                "\"hello world\"",
                Ok(Json::String("hello world".to_owned())),
            ),
        test_parse_list_with_bool:    ("[false]", Ok(Json::Array(vec![Json::Boolean(false)]))),
        test_parse_list_with_null:    ("[null]", Ok(Json::Array(vec![Json::Null]))),
        test_parse_list_with_mixed:    (
                "[1,2,3,false]",
                Ok(Json::Array(vec![
                    Json::Integer(1),
                    Json::Integer(2),
                    Json::Integer(3),
                    Json::Boolean(false),
                ])),
            ),
        test_parse_list_nested: (
                "[[1],null]",
                Ok(Json::Array(vec![
                    Json::Array(vec![Json::Integer(1)]),
                    Json::Null,
                ])),
            ),
        test_parse_dict_empty: ("{}", Ok(Json::Object(HashMap::new()))),
        test_parse_dict_one_item:           (
                "{\"foo\": 1337}",
                Ok(Json::Object(HashMap::from([(
                    "foo".to_owned(),
                    Json::Integer(1337),
                )]))),
            ),
        test_parse_dict_two_items: (
                "{\"foo\": 1337, \"bar\": [69]}",
                Ok(Json::Object(HashMap::from([
                    ("foo".to_owned(), Json::Integer(1337)),
                    ("bar".to_owned(), Json::Array(vec![Json::Integer(69)])),
                ]))),
            ),
        test_parse_extra_input: (
                "truefalse",
                Err(ParseError {
                    offset: 1,
                    message: "Unexpected extra input found".to_owned(),
                }),
            ),
        test_parse_object_fail_1:    (
                "{",
                Err(ParseError {
                    offset: 1,
                    message: "Unexpected end of input".to_owned(),
                }),
            ),
            test_parse_object_fail_2:    (
                "{\"some key\"",
                Err(ParseError {
                    offset: 2,
                    message: "Unexpected end of input".to_owned(),
                }),
            ),
            test_parse_object_fail_3:    (
                "{\"some key\":",
                Err(ParseError {
                    offset: 3,
                    message: "Unexpected end of input".to_owned(),
                }),
            ),
            test_parse_object_fail_4:    (
                "{\"some key\":\"some value\"",
                Err(ParseError {
                    offset: 4,
                    message: "Unexpected end of input".to_owned(),
                }),
            ),
            test_parse_object_fail_5:    (
                "{\"some key\":\"some value\" 3",
                Err(ParseError {
                    offset: 4,
                    message: "Unexpected token `3` in object".to_owned(),
                }),
            ),
            test_parse_object_fail_6: (
                    "{3:\"some value\"",
                Err(ParseError {
                    offset: 1,
                    message: "Cannot parse `3` as string".to_owned(),
                }),
            ),
            test_parse_object_fail_7:(
                "{\"some key\" 3",
                Err(ParseError {
                    offset: 2,
                    message: "Unexpected token `3` in object".to_owned(),
                }),
            ),
            test_parse_object_fail_8:(
                "{3",
                Err(ParseError {
                    offset: 1,
                    message: "Cannot parse `3` as string".to_owned(),
                }),
            ),
            test_parse_array_fail_1:(
                "[",
                Err(ParseError {
                    offset: 1,
                    message: "Unexpected end of input".to_owned(),
                }),
            ),
            test_parse_array_fail_2:(
                "[3",
                Err(ParseError {
                    offset: 2,
                    message: "Unexpected end of input".to_owned(),
                }),
            ),
            test_parse_array_fail_3: (
                "[3,",
                Err(ParseError {
                    offset: 3,
                    message: "Unexpected end of input".to_owned(),
                }),
            ),
            test_parse_array_fail_4:(
                "[3 5",
                Err(ParseError {
                    offset: 2,
                    message: "Unexpected token `5` in array".to_owned(),
                }),
            ),
            test_parse_empty: (
                "",
                Err(ParseError {
                    offset: 0,
                    message: "Unexpected end of input".to_owned(),
                }),
            ),
            test_parse_int_fail: (
                "2222222222222222222222222",
                Err(ParseError {
                    offset: 0,
                    message: "Cannot parse `2222222222222222222222222` as integer".to_owned(),
                }),
            ),
            test_parse_stray_token: (
                "}",
                Err(ParseError {
                    offset: 0,
                    message: "Found unexpected token `}`".to_owned(),
                }),
            ),
    }
}
