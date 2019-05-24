type Key = Vec<u8>;
type Value = Vec<u8>;

struct Slab {
    max_key: Key,
    min_key: Key,
    index: HashMap,
}
