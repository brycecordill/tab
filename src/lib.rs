use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string(config.filename)?;

    Ok(())
}

pub struct Config {
    pub name1: String,
    pub amount: f64,
    pub name2: String,
    pub filename: String,
}
impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();  // Skip the exec name

        let name1 = match args.next() {
            Some(arg) => arg.to_lowercase(),
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
            Some(arg) => arg.to_lowercase(),
            None => return Err("Second name not given")
        };

        // Set the data file (Hardcoded for now)
        let filename = match dirs::document_dir() {
            Some(dir) => format!("{}/tab.data", dir.display()),
            None => return Err("Unable to find 'Documents' directory")
        };

        Ok(Config {name1, amount, name2, filename})
    }
}

