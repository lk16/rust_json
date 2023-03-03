
# Rust playground

Barebone JSON parser

### Example Usage

```sh
cargo run -q '{"hello": [123, false, true, {"foo": null}, 3]}'
# outputs: Object({"hello": Array([Integer(123), Boolean(false), Boolean(true), Object({"foo": Null}), Integer(3)])})
```

### Features
* Tokenizer
* Parser
* Tests

### Missing features
* Support for floats
* Support for string escape sequences

### Features to be added soon
* Moving tokenizer and parser to separate file
* Test Coverage
* Adding missing tests
* CI

### Disclaimer

This package was created to remember how rust packaging works.
The actual code inside this project is probably not rocket science.

### Setup and run project

```sh
# Create project skeleton in a new folder `rust_playground`
cargo new rust_playground

# compile and run code
# -q is to silence build info output
cargo run -q

# compile and run code in separate commands
cargo build
./target/debug/rust_playground
```

### Tools used while developing

```rs
// Tell compiler to not complain about
// dead/unreachable code or unused variables in this crate
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
```

```sh
# Watch rust compiler output while editing
watch -cn 1 -- cargo --color=always run

# Watch rust test output while editing
watch -cn 1 -- cargo --color=always test -q
```
