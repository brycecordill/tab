use std::env;
use std::process;

use tab::config::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // If the print argument is given, call print_tab() and exit
    if args.len() > 1 && args[1] == "-p" {
        tab::print_tab();
    }

    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        tab::print_usage();
        process::exit(1);
    });

    if let Err(e) = tab::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1)
    }
}
