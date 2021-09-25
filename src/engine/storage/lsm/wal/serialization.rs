use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq)]
pub(crate) struct FileHeader {
    pub(crate) stanza: Vec<u8>,
    pub(crate) version: u8,
}

impl FileHeader {
    pub(crate) fn new(stanza: &str, version: u8) -> FileHeader {
        FileHeader {
            stanza: stanza.as_bytes().to_vec(),
            version,
        }
    }
}
