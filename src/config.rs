pub struct Config {
    pub name1: String,
    pub amount: f64,
    pub action: String,
    pub name2: String,
    pub filename: String,
}
impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
        
        let mut args = args.into_iter();

        args.next();  // Skip the exec name

        let name1 = match args.next() {
            Some(arg) => arg.to_lowercase(),
            None => return Err("First name not given")
        };

        let action = match args.next() {
            Some(arg) => arg,
            None => return Err("No action given")
        };
        if !(action == "recv" || action == "owes") {
            return Err("Invalid action")
        }

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

        Ok(Config {name1, amount, action, name2, filename})
    }
}