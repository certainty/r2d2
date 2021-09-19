#[derive(Debug, PartialEq)]
pub enum Command {
    Quit,
    Help,
    Insert(String, String),
    Delete(String),
    Lookup(String),
    ListKeys,
}
