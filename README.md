
# Rust JSON

Barebone JSON parser in Rust

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
* Human readable tokenize- and parse- errors

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
watch -cn 1 -- cargo --color=always run -q

# Watch rust test output while editing
watch -cn 1 -- cargo --color=always test -q
```

[Split Rust code into multiple files](https://rust-classes.com/chapter_4_3.html#chapter-43---organizing-code)

Get test coverage:
```sh
# Install grcov and llvm-tools
cargo install grcov
rustup component add llvm-tools-preview

# Run tests
CARGO_INCREMENTAL=0 \
RUSTFLAGS='-C instrument-coverage' \
LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' \
cargo test

# Generate HTML coverage report
grcov . \
--binary-path ./target/debug/deps/ \
-s . \
-t html \
--branch \
--ignore-not-existing \
--ignore '../*' \
--ignore "/*" \
--ignore "src/main.rs" \
-o cov_html

# Show report in browser
firefox cov_html/index.html

# Print Markdown coverage report to stdout
grcov . \
--binary-path ./target/debug/deps/ \
-s . \
-t markdown \
--branch \
--ignore-not-existing \
--ignore '../*' \
--ignore "/*" \
--ignore "src/main.rs" \
-o /dev/stdout
```

```sh
# Run linter
cargo clippy -- -D warnings
```
