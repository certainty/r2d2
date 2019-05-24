use serde;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path;

type Key = Vec<u8>;
type Value = Vec<u8>;
type Offset = usize;
type Level = u8;

const STANZA: &str = "r2d2::sstable";

struct Slab {
    level: Level,
    max_key: Key,
    min_key: Key,
    path: path::PathBuf,
}

struct SSTable {
    // TODO: think about using a trie instead?
    index: HashMap<Key, Offset>,
    path: path::PathBuf,
}

// On disk representation of SSTable as runs of sorted data
type BackingStore = io::BufWriter<fs::File>;

// The on disk representation of an SSTable is as follows
//
// DATA_BLOCK
// META_BLOCK
// INDEX_BLOCK
// TRAILER

#[derive(Serialize, Deserialize)]
struct Trailer<'a> {
    start_of_meta_block: Offset,
    version: u8,
    stanza: &'a [u8],
}

#[derive(Serialize, Deserialize)]
struct Meta {
    data_size: usize,
    data_block_count: u64,
    index_size: usize,
}

struct Writer {
    file: BackingStore,
}
