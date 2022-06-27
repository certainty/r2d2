mod server;

extern crate env_logger;
extern crate log;
use clap::{AppSettings, Parser};
use r2d2::client::cli::command::repl;

#[derive(Parser, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,

    #[clap(
        short,
        long,
        about,
        help = "The configuration file to use, if it's not in the default location"
    )]
    config: Option<String>,
}

#[derive(Parser, Debug)]
enum SubCommand {
    #[clap(version = "0.1", author = "David K.")]
    Repl(repl::Opts),
}

fn main() {
    pretty_env_logger::init();
    let opts: Opts = Opts::parse();

    let result = match opts.subcmd {
        SubCommand::Repl(opts) => repl::execute(&opts),
    };

    match result {
        Ok(()) => std::process::exit(0),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1)
        }
    }
}
