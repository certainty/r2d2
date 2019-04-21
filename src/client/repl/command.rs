use nom::*;
use nom::types::CompleteStr;

#[derive(Debug, PartialEq)]
pub enum Command {
    Quit,
    Help,
    Insert(String, String),
    Delete(String),
    Lookup(String),
    ListKeys,
    Stats,
}

mod parser {
    use super::*;
    named!(cmd_quit<CompleteStr, Command>, map!(tag_s!("quit"), |_| Command::Quit));

    named!(cmd_insert<CompleteStr, Command>, 
      do_parse!(
        tag_s!("insert") >>
        space >>
        key: alphanumeric >> 
        space >>
        value: alphanumeric >>
        (Command::Insert(key.to_string(), value.to_string()))
      )
    );

    named!(cmd_lookup<CompleteStr, Command>, 
      do_parse!(
        tag_s!("lookup") >>
        space >>
        key: alphanumeric >>
        (Command::Lookup(key.to_string()))
      )
    );

    named!(cmd_delete<CompleteStr, Command>, 
      do_parse!(
        tag_s!("delete") >>
        space >>
        key: alphanumeric >>
        (Command::Delete(key.to_string()))
      )
    );

    named!(cmd_list_keys<CompleteStr, Command>, map!(tag_s!("list_keys") , |_| Command::ListKeys));
    named!(cmd_stats<CompleteStr, Command>, map!(tag_s!("stats") , |_| Command::Stats));
    named!(cmd_help<CompleteStr, Command>, map!(tag_s!("help") , |_| Command::Help));

    named!(pub parse_command<CompleteStr, Command>,
      do_parse!(
        tag!(":") >> 
        cmd: alt_complete!(cmd_quit | cmd_lookup | cmd_delete | cmd_insert | cmd_list_keys | cmd_stats | cmd_help) >> 
        (cmd))
    );
  }

  pub fn parse(input: &str) -> Result<Command, String> {
    match parser::parse_command(CompleteStr(input.trim())) {
      Ok((CompleteStr(""), cmd)) => Result::Ok(cmd),
      _ => Result::Err(String::from("Unknown command"))
    }
  }


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_ignored_whitespace() {
    assert_eq!(parse(&String::from("   :list_keys    ")), Ok(Command::ListKeys))
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
    assert_eq!(
      parse(&String::from(":insert")), 
      Err(String::from("Unknown command"))
    )
  }

  #[test]
  fn parse_insert_fails_when_value_is_missing() {
    assert_eq!(
      parse(&String::from(":insert bar")), 
      Err(String::from("Unknown command"))
    )
  }

  #[test]
  fn parse_fails_with_appended_garbage() {
    assert_eq!(
      parse(&String::from(":insert foo bar garbage")),
      Err(String::from("Unknown command"))
    )
  }

  #[test]
  fn parse_delete_succeeds(){
    assert_eq!(
      parse(&String::from(":delete foo")),
      Ok(Command::Delete(String::from("foo"))));
  }

#[test]
  fn parse_delete_fails_when_key_is_missing(){
    assert_eq!(
      parse(&String::from(":delete  ")),
      Err(String::from("Unknown command"))
    )
  }

  #[test]
  fn parse_delete_fails_with_appended_garbage(){
    assert_eq!(
      parse(&String::from(":delete foo garbagelfdsjlkf")),
      Err(String::from("Unknown command"))
    )
  }

  #[test]
  fn parse_lookup_succeeds(){
    assert_eq!(
      parse(&String::from(":lookup foo")),
      Ok(Command::Lookup(String::from("foo")))
    )
  }

#[test]
  fn parse_lookup_fails_when_key_is_missing(){
    assert_eq!(
      parse(&String::from(":lookup  ")),
      Err(String::from("Unknown command"))
    )
  }

  #[test]
  fn parse_lookup_fails_with_appended_garbage(){
    assert_eq!(
      parse(&String::from(":lookup foo garbagelfdsjlkf")),
      Err(String::from("Unknown command"))
    )
  }

  #[test]
  fn parse_list_keys_succeeds() {
    assert_eq!(
      parse(&String::from(":list_keys")), 
      Ok(Command::ListKeys)
    )
  }

#[test]
  fn parse_help_succeeds() {
    assert_eq!(
      parse(&String::from(":help")), 
      Ok(Command::Help)
    )
  }

  #[test]
  fn parse_quit_succeeds() {
    assert_eq!(
      parse(&String::from(":quit")),
      Ok(Command::Quit)
    )
  }
}
