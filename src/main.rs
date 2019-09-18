use std::env;
use std::process;

use tab::config::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = tab::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1)
    }
}
