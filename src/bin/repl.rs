extern crate env_logger;
extern crate log;
extern crate r2d2_lib;

use r2d2_lib::client::repl;
use r2d2_lib::engine;

fn main() {
  env_logger::init();
  let mut engine = engine::default::new();

  repl::run(&mut engine);
}
