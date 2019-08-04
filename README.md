# Key value store in Rust

This is an implementation of a key-value store in Rust.
It is currently for educational purposes only to:

* Learn Rust
* Get a deeper understanding of log structured merge trees
* Play with the CURP replication protocol

## Run the repl to interact with the store

```
cargo run --bin r2d2 repl
```

## Run the tests

```
RUST_TEST_THREADS=1 cargo test -- --nocapture
```
