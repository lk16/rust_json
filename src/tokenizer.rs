use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    ArrayEnd,
    ArrayStart,
    Colon,
    Comma,
    False,
    Number,
    Null,
    ObjectEnd,
    ObjectStart,
    String,
    True,
    Whitespace,
}

#[derive(Debug, PartialEq)]
pub struct TokenizeError {
    pub offset: usize,
    pub message: String,
}

impl TokenizeError {
    fn new(offset: usize, message: String) -> Self {
        Self { offset, message }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub type_: TokenType,
    pub value: String,
    pub offset: usize,
}

impl Token {
    pub fn new(type_: TokenType, value: &str, offset: usize) -> Self {
        Self {
            type_,
            value: value.to_owned(),
            offset,
        }
    }

    pub fn len(&self) -> usize {
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
                Some('-') => self.tokenize_number(),
                Some(c) => {
                    if c.is_ascii_digit() {
                        self.tokenize_number()
                    } else if c.is_ascii_whitespace() {
                        self.tokenize_whitespace()
                    } else {
                        Err(TokenizeError::new(
                            self.offset,
                            "Unhandled character".to_owned(),
                        ))
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
        Err(TokenizeError::new(
            self.offset,
            format!("Expected literal `{}`", literal),
        ))
    }

    fn tokenize_number(&self) -> Result<Token, TokenizeError> {
        let re = Regex::new(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?").unwrap();

        let found = re.find_at(&self.input, self.offset);

        match found {
            None => Err(TokenizeError::new(
                self.offset,
                "Cannot parse number".to_owned(),
            )),
            Some(found) => {
                let value = found.as_str();
                Ok(Token::new(TokenType::Number, value, self.offset))
            }
        }
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

pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenizeError> {
    Tokenizer::new(input).tokenize()
}

#[cfg(test)]
mod tests {
    use crate::tokenize;
    use crate::tokenizer::Token;
    use crate::tokenizer::TokenType;
    use crate::tokenizer::TokenizeError;

    macro_rules! tokenizer_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(tokenize(input), expected)

            }
        )*
        }
    }

    tokenizer_tests! {
        test_tokenize_null:    ("null", Ok(vec![Token::new(TokenType::Null, "null", 0)])),
        test_tokenize_true:    ("true", Ok(vec![Token::new(TokenType::True, "true", 0)])),
        test_tokenize_false:    ("false", Ok(vec![Token::new(TokenType::False, "false", 0)])),
        test_tokenize_colon:    (":", Ok(vec![Token::new(TokenType::Colon, ":", 0)])),
        test_tokenize_array_start:    ("[", Ok(vec![Token::new(TokenType::ArrayStart, "[", 0)])),
        test_tokenize_array_end:    ("]", Ok(vec![Token::new(TokenType::ArrayEnd, "]", 0)])),
        test_tokenize_object_start:    ("{", Ok(vec![Token::new(TokenType::ObjectStart, "{", 0)])),
        test_tokenize_object_end:    ("}", Ok(vec![Token::new(TokenType::ObjectEnd, "}", 0)])),
        test_tokenize_comma:    (",", Ok(vec![Token::new(TokenType::Comma, ",", 0)])),
        test_tokenize_number:    ("1234", Ok(vec![Token::new(TokenType::Number, "1234", 0)])),
        test_tokenize_number_negative:    ("-1234", Ok(vec![Token::new(TokenType::Number, "-1234", 0)])),
        test_tokenize_number_decimal:    ("3.1415926", Ok(vec![Token::new(TokenType::Number, "3.1415926", 0)])),
        test_tokenize_number_exponent:    ("69234.2423432E78", Ok(vec![Token::new(TokenType::Number, "69234.2423432E78", 0)])),
        test_tokenize_whitespace: (
                " \n\r ",
                Ok(vec![Token::new(TokenType::Whitespace, " \n\r ", 0)]),
            ),
        test_tokenize_string: (
                "\"Hello world\"",
                Ok(vec![Token::new(TokenType::String, "\"Hello world\"", 0)]),
            ),
        test_tokenize_many: (
                "123 {} [] , : \"a b\" null\nfalsetrue",
                Ok(vec![
                    Token::new(TokenType::Number, "123", 0),
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
        test_tokenize_broken: (
                "broken",
                Err(TokenizeError {
                    offset: 0,
                    message: "Unhandled character".to_owned(),
                }),
            ),
        test_tokenize_unclosed_string: (
                "\"no closing quote",
                Err(TokenizeError {
                    offset: 0,
                    message: "No string-terminating quote found".to_owned(),
                }),
            ),
        test_tokenize_broken_false: (
                "foo",
                Err(TokenizeError {
                    offset: 0,
                    message: "Expected literal `false`".to_owned(),
                }),
            ),
    }

    #[test]
    fn test_tokenize() {
        let cases: Vec<(&str, Result<Vec<Token>, TokenizeError>)> = vec![];

        for case in cases.iter() {
            assert_eq!(tokenize(case.0), case.1)
        }
    }
}
