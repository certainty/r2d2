use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::ops::Deref;
use std::string::FromUtf8Error;

#[repr(transparent)]
#[derive(Debug, Clone, Ord, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
/// A key is an arbitrary sequence of bytes
pub struct Key(Vec<u8>);

impl Eq for Key {}

impl Key {
    pub fn new(data: Vec<u8>) -> Key {
        Key(data)
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.clone()
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.0
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl TryFrom<Key> for String {
    type Error = FromUtf8Error;

    fn try_from(key: Key) -> Result<Self, Self::Error> {
        String::from_utf8(key.to_vec())
    }
}

impl TryFrom<&Key> for String {
    type Error = FromUtf8Error;

    fn try_from(key: &Key) -> Result<Self, Self::Error> {
        String::from_utf8(key.to_vec())
    }
}

impl AsRef<[u8]> for Key {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl From<&[u8]> for Key {
    fn from(v: &[u8]) -> Self {
        Self(v.to_vec())
    }
}

impl From<&str> for Key {
    fn from(s: &str) -> Self {
        Self::new(s.as_bytes().to_vec())
    }
}

impl From<String> for Key {
    fn from(s: String) -> Self {
        Self::new(s.as_bytes().to_vec())
    }
}

impl Deref for Key {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
