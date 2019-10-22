use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process;

pub mod config;

#[cfg(test)]
mod tests;

type Config = config::Config;

/// Prints out the data stored in the data file, then exits
pub fn print_tab() {
    // Set the data file (Hardcoded for now)
    let filename = match dirs::document_dir() {
        Some(dir) => format!("{}/tab.data", dir.display()),
        None => {
            eprintln!("Unable to find 'Documents' directory");
            process::exit(1);
        }
    };

    let file_data = fs::read_to_string(&filename).unwrap_or_else(|_| {
        eprintln!("Unable to open the tab file.  Has it been created yet?");
        process::exit(1);
    });

    let file_data = file_data.replace("_", " ");

    let mut word_iter = file_data.split_ascii_whitespace();


    let name1 = word_iter.next().unwrap();
    let name2 = word_iter.next().unwrap();
    let amount: f64 = word_iter.next().unwrap().parse().unwrap();

    if amount >= 0.0 {
        println!("{} owes {} ${}", name1, name2, amount);
    }
    else {
        println!("{} owes {} ${}", name2, name1, -1.0 * amount);
    }

    process::exit(0);
}

/// Prints a usage message to stderr
pub fn print_usage() {
    eprintln!("\nUSAGE: tab name1 action amount name2\n");
    eprintln!("where action is either paid or owes \
        and amount is the amount owed/paid");
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Create the data file if it doesn't exist
    if !Path::new(&config.filename).exists() {
        File::create(&config.filename)?;
    }
    let mut data = fs::read_to_string(&config.filename)?;

    // 'Sort' the name combo so it is consistent
    let name_combo = generate_name_combo(&config);

    let (start_i, end_i) = match get_indices(&data, &name_combo) {
        Some((s, e)) => (s, e),
        // If the combo is not found, add it to the file and end the program
        None => return append_new(&config, &name_combo),
    };

    let new_line = update_line(&config, &data[start_i..end_i], &name_combo);

    // Replace the old line with the new line
    data.replace_range(start_i..(end_i + 1), &new_line);

    // Overwrite the old data with the modified data
    let mut out_file = fs::File::create(&config.filename)?;
    out_file.write(&data.as_bytes())?;

    Ok(())
}

/// Returns an updated version of the line based on the current
/// amount and the amount to add
fn update_line(config: &Config, old_ln: &str, name_combo: &str) -> String {
    let current_owed = parse_line(&old_ln);
    let to_add = calculate_tab(&config);

    format!("{} {:.2}\n", &name_combo, current_owed + to_add)
}

/// Returns the start and end indices of the line matching `name_combo`
/// relative to the `data` str.
/// Returns `None` if the `name_combo` is not found
fn get_indices(data: &str, name_combo: &str) -> Option<(usize, usize)> {
    let start_index = match data.find(&name_combo) {
        Some(num) => num,
        None => return None,
    };

    let mut end_index = start_index;
    for (i, ch) in data[start_index..].char_indices() {
        if ch == '\n' {
            end_index = i;
            break;
        }
    }

    Some((start_index, end_index))
}

/// Generates and returns a `name_combo` String by sorting alphabetically
fn generate_name_combo(config: &Config) -> String {
    if config.name1 < config.name2 {
        format!("{}_{}", config.name1, config.name2)
    } else {
        format!("{}_{}", config.name2, config.name1)
    }
}

/// Appends a new line to the end of the file with correct formatting
fn append_new(config: &Config, name_combo: &str) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new().append(true).open(&config.filename)?;

    let new_line = format!("{} {}\n", name_combo, calculate_tab(&config));

    file.write(&new_line.as_bytes())?;
    Ok(())
}

/// Returns the amount owed based on the given line
fn parse_line(ln: &str) -> f64{
    let mut start_char = 0;
    
    // Iterate until the amount is found
    for (i, ch) in ln.char_indices() {
        if ch == ' ' {
            start_char = i;
            break;
        }
    }

    // Parse the amount
    let current_tab: f64 = ln.get(start_char..).unwrap().trim().parse().unwrap();
    current_tab
    
}

/// Determines whether to add or subtract the `config.amount` value based
/// on the action supplied and the order of the names.
/// Can only be called with the actions `"paid"` or `"owes"` 
fn calculate_tab(config: &Config) -> f64 {
    assert!(config.action == "paid" || config.action == "owes");

    if config.action == "owes" {
        // If name1 owes name2 and name1 is first in name_combo
        if config.name1 < config.name2 {
            return config.amount
        }
        // If name_combo is flipped from the action, return the opposite
        config.amount * -1.0
    } else {  // If action == "paid"
        // If name1 recieved from name2 and name1 is first in name_combo
        if config.name1 < config.name2 {
            return config.amount * -1.0
        }
        // If name_combo is flipped from the action, return the opposite
        config.amount
    }
}
