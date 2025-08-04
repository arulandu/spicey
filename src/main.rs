use std::env;
use std::process;

mod ast;
mod parser;
use parser::parse_file;
mod ngspice;
use ngspice::NgSpiceManager;

mod repl;
use repl::run;


fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|_| {
        eprintln!("Problem parsing arguments");
        process::exit(1);
    });

    parse_file(&config.input_file);

    run();
}

struct Config {
    input_file: String
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        if let Some(input_file) = args.next() {
            Ok(Config { input_file })
        } else {
            Err("No input file provided")
        }

    }
}
