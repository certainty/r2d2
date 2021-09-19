//! The repl module provides a simple interactive client to try things out
//!
//! This is a basic read-eval-print loop that reads line based commands and evals
//! them with the currently active engine. This is intdended to be used for debug
//! purposes only.

extern crate rustyline;
use crate::engine::{Engine, Key};

mod command;
mod command_parser;

use command::Command;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::convert::TryInto;
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

impl Output {
    pub fn message<M: Into<String>>(m: M) -> Output {
        Output::Message(m.into())
    }

    pub fn error<M: Into<String>>(m: M) -> Output {
        Output::Error(m.into())
    }
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

fn eval(cmd: Command, engine: &mut Engine) -> Output {
    match cmd {
        Command::Quit => Output::Break,

        Command::Insert(key, value) => match engine.set(key, value) {
            Ok(_) => Output::message("OK <>"),
            Err(msg) => Output::error(format!("{:?}", msg)),
        },

        Command::Delete(key) => match engine.del(&Key::from(key)) {
            Ok(Some(value)) => {
                let string_value: String = value.try_into().unwrap();
                Output::message(format!("OK <{}>", string_value))
            }
            Ok(None) => Output::message("OK <>"),
            Err(msg) => Output::error(format!("{:?}", msg)),
        },

        Command::Lookup(key) => match engine.get(&Key::from(key)) {
            Ok(Some(value)) => Output::Message(format!(
                "OK <{}>",
                String::from_utf8(value.to_vec()).unwrap()
            )),
            Ok(None) => Output::message("OK <>"),
            Err(msg) => Output::error(format!("{:?}", msg)),
        },

        Command::ListKeys => {
            let string_keys: Result<Vec<String>, _> =
                engine.iter().map(|(k, _v)| TryInto::try_into(k)).collect();
            Output::Message(format!("OK <{}>", string_keys.unwrap().join(", ")))
        }

        Command::Help => Output::message(
            "
    The following comands are available

    :help\t\t\tShow this help
    :list_keys\t\t\tList the currently known keys
    :insert <key> <value>\tInsert a new key value pair
    :lookup <key>\t\tFind the value for the given <key>
    :delete <key>\t\tDelete a key that has been previously inserted
    :quit\t\t\tExit the repl
    ",
        ),
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
