use std::env;
use std::process;

pub struct Config {
    pub name1: String,
    pub amount: f64,
    pub name2: String,
}
impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();  // Skip the exec name

        let name1 = match args.next() {
            Some(arg) => arg,
            None => return Err("First name not given")
        };

        let amount = match args.next() {
            Some(arg) => arg,
            None => return Err("No amount given")
        };
        let amount: f64 = match amount.parse() {
            Ok(num) => num,
            Err(_) => return Err("Failed to parse 'amount'"),
        };

        let name2 = match args.next() {
            Some(arg) => arg,
            None => return Err("Second name not given")
        };

        Ok(Config {name1, amount, name2})
    }
}

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
}
