use super::command::Command;
use thiserror::Error;

#[derive(Error, Clone, Debug, PartialEq)]
pub enum Error {
    #[error("Invalid command")]
    ParseError,
}
pub type Result<T> = std::result::Result<T, Error>;

pub fn parse(input: &str) -> Result<Command> {
    let parts = input.split_whitespace().collect::<Vec<&str>>();

    match parts[..] {
        [":insert", key, value] => Ok(Command::Insert(key.into(), value.into())),
        [":lookup", key]  => Ok(Command::Lookup(key.into())),
        [":delete", key]  => Ok(Command::Delete(key.into())),
        [":list_keys"] => Ok(Command::ListKeys),
        [":help"] => Ok(Command::Help),
        [":quit"] => Ok(Command::Quit),
        _ => Err(Error::ParseError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ignored_whitespace() {
        assert_eq!(
            parse(&String::from("   :list_keys    ")),
            Ok(Command::ListKeys)
        )
    }

    #[test]
    fn parse_insert_succeeds() {
        assert_eq!(
            parse(&String::from(":insert foo bar")),
            Ok(Command::Insert(String::from("foo"), String::from("bar")))
        )
    }

    #[test]
    fn parse_insert_fails_when_key_is_missing() {
        assert_eq!(parse(&String::from(":insert")), Err(Error::ParseError))
    }

    #[test]
    fn parse_insert_fails_when_value_is_missing() {
        assert_eq!(parse(&String::from(":insert bar")), Err(Error::ParseError))
    }


    #[test]
    fn parse_delete_succeeds() {
        assert_eq!(
            parse(&String::from(":delete foo")),
            Ok(Command::Delete(String::from("foo")))
        );
    }

    #[test]
    fn parse_delete_fails_when_key_is_missing() {
        assert_eq!(parse(&String::from(":delete  ")), Err(Error::ParseError))
    }

    #[test]
    fn parse_lookup_succeeds() {
        assert_eq!(
            parse(&String::from(":lookup foo")),
            Ok(Command::Lookup(String::from("foo")))
        )
    }

    #[test]
    fn parse_lookup_fails_when_key_is_missing() {
        assert_eq!(parse(&String::from(":lookup  ")), Err(Error::ParseError))
    }

    #[test]
    fn parse_list_keys_succeeds() {
        assert_eq!(parse(&String::from(":list_keys")), Ok(Command::ListKeys))
    }

    #[test]
    fn parse_help_succeeds() {
        assert_eq!(parse(&String::from(":help")), Ok(Command::Help))
    }

    #[test]
    fn parse_quit_succeeds() {
        assert_eq!(parse(&String::from(":quit")), Ok(Command::Quit))
    }
}
