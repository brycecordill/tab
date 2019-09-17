use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::fs::OpenOptions;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Create the data file if it doesn't exist
    if !Path::new(&config.filename).exists() {
        File::create(&config.filename)?;
    }
    let mut data = fs::read_to_string(&config.filename)?;

    // 'Sort' the name combo so it is consistent
    let name_combo = if config.name1 < config.name2 {
        format!("{}_{}", config.name1, config.name2)
    } else {
        format!("{}_{}", config.name1, config.name2)
    };

    let start_index = match data.find(&name_combo) {
        Some(num) => num,
        // If the combo is not found, add it to the file and end the program
        None => return append_new(&config, &name_combo),
    };

    let mut end_index = start_index;
    for (i, ch) in data[start_index..].char_indices() {
        if ch == '\n' {
            end_index = i;
            break;
        }
    }

    let current_owed = parse_tab_line(&data[start_index..end_index]);
    let to_add = calculate_tab(&config);

    let final_line = format!("{} {}", name_combo, current_owed + to_add);

    data.replace_range(start_index..(end_index + 1), &final_line);

    let mut out_file = fs::File::create(&config.filename)?;
    out_file.write(&data.as_bytes())?;


    Ok(())
}

fn append_new(config: &Config, name_combo: &str) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new().append(true).open(&config.filename)?;

    let new_line = format!("{} {}", name_combo, calculate_tab(&config));

    file.write(&new_line.as_bytes())?;
    Ok(())
}

fn parse_tab_line(ln: &str) -> f64{
    let mut start_char = 0;
    
    for (i, ch) in ln.char_indices() {
        if ch == ' ' {
            start_char = i;
            break;
        }
    }

    let current_tab: f64 = ln.get(start_char..).unwrap().trim().parse().unwrap();
    current_tab
    
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

