
[![Rust](https://github.com/certainty/r2d2/actions/workflows/rust.yml/badge.svg)](https://github.com/certainty/r2d2/actions/workflows/rust.yml)

# Key value store in Rust

This is an implementation of a key-value store in Rust.
It is currently for educational purposes only to:

* Learn Rust
* Get a deeper understanding of log structured merge trees
* Play with the CURP replication protocol

## Run the repl to interact with the store

```
make repl
```

## Run the tests

```
make test
```

## Todo

### LSM

- [ ] Use sstables correctly (currently I believe I don't always find the right one)
- [ ] Implement proper handling of tombstones in sstables
- [ ] Use a (concurrent) skiplist for the memtable and different write threads 

### Distribution
- [ ] Add grpc based communication
- [ ] Add vnodes and consistent hashing to identify the correct node
- [ ] Add gossping to distribute state


### Repl
- [ ] Add text UI to display cluster properties and interact with the server
