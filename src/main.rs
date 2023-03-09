use crate::parser::parse;
use crate::tokenizer::tokenize;

mod parser;
mod tokenizer;

use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <json string>", args[0]);
        std::process::exit(1);
    }

    let tokenized = tokenize(&args[1]);

    match tokenized {
        Ok(tokens) => {
            let parsed = parse(tokens);
            match parsed {
                Ok(json) => {
                    println!("{:?}", json);
                    ExitCode::from(0)
                }
                Err(error) => {
                    println!("Parse Error at token {}: {}", error.offset, error.message);
                    ExitCode::from(1)
                }
            }

        }
        Err(error) => {
            println!(
                "Tokenize Error at offset {}: {}",
                error.offset, error.message
            );
            ExitCode::from(1)
        },
    }
}
