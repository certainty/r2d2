# Key value store in rust
Learning project that implements a distibuted kv store that uses CURP for replication.

## Run the repl to interact with the store

```
cargo run --bin r2d2 repl
```

## Run the tests

```
RUST_TEST_THREADS=1 cargo test -- --nocapture
```