//! The repl module provides a simple interactive client to try things out
//!
//! This is a basic read-eval-print loop that reads line based commands and evals
//! them with the currently active engine. This is intdended to be used for debug
//! purposes only.

mod command;
extern crate rustyline;
use crate::engine::{Engine, Key, Value};
use command::Command;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::result::Result;
use termion::color;

const HISTORY_FILE: &str = ".r2d2_history";

enum Input {
  Break,
  Cmd(Command),
}

enum Output {
  Break,
  Error(String),
  Message(String),
}

pub fn run(engine: &mut Engine) {
  let mut editor = Editor::<()>::new();
  editor.load_history(HISTORY_FILE).ok();
  println!("r2d2 repl :: use :help to get help and :quit to exit");

  loop {
    match read(&mut editor) {
      Ok(Input::Cmd(cmd)) => match eval(cmd, engine) {
        Output::Break => break,
        output => print(output),
      },
      Err(msg) => print(Output::Error(msg)),
      Ok(Input::Break) => break,
    }
  }

  editor.save_history(HISTORY_FILE).unwrap();
}

fn read(editor: &mut Editor<()>) -> Result<Input, String> {
  let readline = editor.readline(">> ");
  match readline {
    Ok(line) => {
      editor.add_history_entry(line.as_ref());
      match command::parse(&line) {
        Ok(cmd) => Result::Ok(Input::Cmd(cmd)),
        Err(msg) => Result::Err(msg),
      }
    }
    Err(ReadlineError::Interrupted) => Result::Ok(Input::Break),
    Err(ReadlineError::Eof) => Result::Ok(Input::Break),
    Err(err) => Result::Err(err.to_string()),
  }
}

fn eval(cmd: Command, engine: &mut Engine) -> Output {
  match cmd {
    Command::Quit => Output::Break,

    Command::Stats => Output::Message(String::from("Printing statistics")),

    Command::Insert(key, value) => {
      match engine.insert(Key::from_string(key), Value::from_string(value)) {
        Ok(_) => Output::Message(String::from("OK <>")),
        Err(msg) => Output::Error(msg),
      }
    }

    Command::Delete(key) => match engine.delete(Key::from_string(key)) {
      Ok(Some(value)) => Output::Message(format!("OK <{:?}>", value_to_str(&value))),
      Ok(None) => Output::Message(String::from("OK <>")),
      Err(msg) => Output::Error(msg),
    },

    Command::Lookup(key) => match engine.lookup(Key::from_string(key)) {
      Ok(Some(value)) => Output::Message(format!("OK <{:?}>", value_to_str(value))),
      Ok(None) => Output::Message(String::from("OK <>")),
      Err(msg) => Output::Error(msg),
    },

    Command::ListKeys => match engine.list_keys() {
      Ok(keys) => Output::Message(format!(
        "OK <{:?}>",
        keys.into_iter().map(|k| key_to_str(&k))
      )),
      Err(msg) => Output::Error(msg),
    },

    Command::Help => Output::Message(String::from(
      "
    The following comands are available

    :help\t\t\tShow this help
    :stats\t\t\tShow statistics 
    :list_keys\t\t\tList the currently known keys
    :insert <key> <value>\tInsert a new key value pair
    :lookup <key>\t\tFind the value for the given <key>
    :delete <key>\t\tDelete a key that has been previously inserted
    :quit\t\t\tExit the repl
    ",
    )),
  }
}

fn value_to_str(value: &Value) -> String {
  String::from_utf8(value.as_bytes().clone()).unwrap()
}

fn key_to_str(key: &Key) -> String {
  String::from_utf8(key.as_bytes().clone()).unwrap()
}

fn print(output: Output) {
  match output {
    Output::Message(msg) => println!("{}", msg),
    Output::Error(msg) => println!(
      "{}Error: {}{}",
      color::Fg(color::Red),
      msg,
      color::Fg(color::Reset)
    ),
    Output::Break => (),
  }
}
