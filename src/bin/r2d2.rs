extern crate env_logger;
extern crate log;
extern crate r2d2_lib;
extern crate clap;

use std::path::PathBuf;
use r2d2_lib::client::repl;
use r2d2_lib::engine;
use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("r2d2")
                    .author("David Krentzlin <david.krentzlin@gmail.com>")
                    .about("Simple key-value store to explore the implementation")
                    .version("0.0.1")
                    .arg(Arg::with_name("config")
                         .short("c")
                         .value_name("FILE")
                         .help("Specified the path to the configuration file")
                         .takes_value(true)
                         .default_value("~/.rd2d.conf"))
                    .subcommand(SubCommand::with_name("repl")
                                .version("0.0.1"))
                    .get_matches();

    if let Some(_subcommand_matches) = matches.subcommand_matches("repl") {
        start_repl();
    } else {
      println!("No such command");
    }
}

fn start_repl() {
  env_logger::init();
  let mut engine = engine::default::new(PathBuf::from("/tmp"));

  repl::run(&mut engine);
}
