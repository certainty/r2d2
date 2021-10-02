use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::ops::Deref;
use std::string::FromUtf8Error;

#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
#[repr(transparent)]
/// A value is an arbitrary sequence of bytes
pub struct Value(Vec<u8>);

impl Value {
    pub fn new(data: Vec<u8>) -> Value {
        Value(data)
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

impl TryFrom<Value> for String {
    type Error = FromUtf8Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        String::from_utf8(value.to_vec())
    }
}

impl TryFrom<&Value> for String {
    type Error = FromUtf8Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        String::from_utf8(value.to_vec())
    }
}

impl AsRef<[u8]> for Value {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<&[u8]> for Value {
    fn from(v: &[u8]) -> Self {
        Self(v.to_vec())
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Self::new(s.as_bytes().to_vec())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Self::new(s.as_bytes().to_vec())
    }
}

impl Deref for Value {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
