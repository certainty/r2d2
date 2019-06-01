//! The repl module provides a simple interactive client to try things out
//!
//! This is a basic read-eval-print loop that reads line based commands and evals
//! them with the currently active engine. This is intdended to be used for debug
//! purposes only.

extern crate itertools;
extern crate rustyline;
use crate::engine::{Engine, Key, Value};

mod command;
mod command_parser;

use command::Command;
use itertools::Itertools;
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

// TODO: add completer for commands
pub fn run(engine: &mut impl Engine) {
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
            let l: &str = &line;
            editor.add_history_entry(l);
            match command_parser::parse(l) {
                Ok(cmd) => Result::Ok(Input::Cmd(cmd)),
                Err(_) => Result::Err(String::from("Invalid command")),
            }
        }
        Err(ReadlineError::Interrupted) => Result::Ok(Input::Break),
        Err(ReadlineError::Eof) => Result::Ok(Input::Break),
        Err(err) => Result::Err(err.to_string()),
    }
}

fn eval(cmd: Command, engine: &mut impl Engine) -> Output {
    match cmd {
        Command::Quit => Output::Break,

        Command::Stats => Output::Message(String::from("Printing statistics")),

        Command::Insert(key, value) => {
            match engine.set(Key::from_string(&key), Value::from_string(&value)) {
                Ok(_) => Output::Message(String::from("OK <>")),
                Err(msg) => Output::Error(format!("{:?}", msg)),
            }
        }

        Command::Delete(key) => match engine.del(&Key::from_string(&key)) {
            Ok(Some(value)) => {
                Output::Message(format!("OK <{}>", String::from_utf8(value.data).unwrap()))
            }
            Ok(None) => Output::Message(String::from("OK <>")),
            Err(msg) => Output::Error(format!("{:?}", msg)),
        },

        Command::Lookup(key) => match engine.get(&Key::from_string(&key)) {
            Ok(Some(value)) => {
                Output::Message(format!("OK <{}>", String::from_utf8(value.data).unwrap()))
            }
            Ok(None) => Output::Message(String::from("OK <>")),
            Err(msg) => Output::Error(format!("{:?}", msg)),
        },

        Command::ListKeys => match engine.keys() {
            Ok(keys) => Output::Message(format!(
                "OK <{}>",
                keys.iter()
                    .map(|k| std::str::from_utf8(&k.data).unwrap())
                    .join(", ")
            )),

            Err(msg) => Output::Error(format!("{:?}", msg)),
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
