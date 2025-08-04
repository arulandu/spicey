use libc::c_int;
use std::io::Write;
use colored::Colorize;

use super::parser::parse_file;
use super::ngspice::{NgSpice, NgSpiceManager, VecInfoAll, VecValuesAll};
use super::Config;

struct Manager {
}

impl NgSpiceManager for Manager {
    fn send_char(&mut self, msg: String, id: i32) {
        let m = match msg.split_once(' ') {
            Some((t, m)) => match t {
                "stdout" => m.purple(),
                "stderr" => m.red(),
                _ => m.purple().strikethrough(),
            }
            None => msg.purple().strikethrough(),
        };
        println!("{}", m);
    }

    fn send_data(&mut self, vecvaluesall: VecValuesAll, count: i32, id: i32) {
        println!("send_data {}", vecvaluesall.vecsa.iter().map(|x| x.name.clone()).collect::<Vec<_>>().join(", "));
    }

    fn send_init_data(&mut self, vecinfoall: VecInfoAll, id: i32) {
        println!("send_init_data {}", vecinfoall.vecs.iter().map(|x| x.name.clone()).collect::<Vec<_>>().join(", "));
    }
}

pub fn run(config: &Config) {
    println!("{}", "Spicey v0.0.1: NgSpice REPL".bold().green());

    let ng = NgSpice::new(None).unwrap();
    let manager = Manager { };
    ng.init(Some(manager)).unwrap();

    let ast = parse_file(&config.input_file).unwrap();
    ng.load(&ast).unwrap();

    let mut buffer = String::new();
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        buffer.clear();
        let _ = std::io::stdin().read_line(&mut buffer).unwrap();

        match buffer.as_str().split_once("\n") {
            Some((f, _)) => {
                ng.command(f.trim()).map_err(|e| {
                    println!("{}", e.red());
                });
            },
            None => {}
        }
    }
}