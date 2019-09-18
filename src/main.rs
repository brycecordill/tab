use std::env;
use std::process;

extern crate dirs;
use tab::config::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = tab::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1)
    }
}
