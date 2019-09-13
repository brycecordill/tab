use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::fs::OpenOptions;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if !Path::new(&config.filename).exists() {
        File::create(&config.filename)?;
    }
    let data = fs::read_to_string(&config.filename)?;

    // 'Sort' the name combo so it is consistent
    let name_combo = if config.name1 < config.name2 {
        format!("{}_{}", config.name1, config.name2)
    } else {
        format!("{}_{}", config.name1, config.name2)
    };

    let i = match data.find(&name_combo) {
        Some(num) => num,
        None => return append_new(&config, &name_combo),
    };

    Ok(())
}

fn append_new(config: &Config, name_combo: &str) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new().append(true).open(&config.filename)?;

    let new_line = format!("{} {}", name_combo, calculate_tab(&config));

    file.write(&new_line.as_bytes())?;
    Ok(())
}

// TODO Comment this or rewrite to make more clear
fn calculate_tab(config: &Config) -> f64 {
    assert!(config.action == "recv" || config.action == "owes");

    if config.action == "recv" {
        if config.name1 < config.name2 {
            return config.amount
        }
        config.amount * -1.0
    } else {  // If action == "owes"
        if config.name1 < config.name2 {
            return config.amount * -1.0
        }
        config.amount
    }
}

pub struct Config {
    pub name1: String,
    pub amount: f64,
    pub action: String,
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

        let action = match args.next() {
            Some(arg) => arg,
            None => return Err("No action given")
        };
        if action != "recv" || action != "owes" {
            return Err("Invalid action")
        }

        let name2 = match args.next() {
            Some(arg) => arg.to_lowercase(),
            None => return Err("Second name not given")
        };

        // Set the data file (Hardcoded for now)
        let filename = match dirs::document_dir() {
            Some(dir) => format!("{}/tab.data", dir.display()),
            None => return Err("Unable to find 'Documents' directory")
        };

        Ok(Config {name1, amount, action, name2, filename})
    }
}

