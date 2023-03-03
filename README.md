
# Rust playground

This package was created to remember how rust packaging works.
The actual code inside this project is probably not rocket science.

---

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

---

### Tools used while developing

```rs
// Tell compiler to not complain about
// unused code or variables in this crate
#![allow(dead_code)]
#![allow(unused_variables)]
```

```sh
# Watch rust compiler output while editing
watch -cn 1 -- cargo --color=always run

# Watch rust test output while editing
watch -cn 1 -- cargo --color=always test -q
```
