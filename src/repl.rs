use libc::c_int;
use std::io::Write;
use colored::Colorize;
use super::ngspice::{NgSpice, NgSpiceManager};


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
}

pub fn run() {
    println!("{}", "Spicey v0.0.1: NgSpice REPL".bold().green());

    let ng = NgSpice::new(None).unwrap();
    let manager = Manager { };
    ng.init(Some(manager)).unwrap();

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