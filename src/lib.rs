use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    /// Parse the arguments and construct Config.
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = Config::get_case_sensitive(&args)?;

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }

    /// Get Config field - case_sensitive, through arg or env variable.
    ///
    /// The default is that arg take precedence over env variable.
    pub fn get_case_sensitive(args: &[String]) -> Result<bool, &'static str> {
        // If the third arg is exist or not
        if args.len() == 4 {
            if args[3] == "true" {
                Ok(true)
            } else if args[3] == "false" {
                Ok(false)
            } else {
                // The format of the arg is illegal
                Err("the third argument must be true or false")
            }
        } else {
            // Decided by env variable if the third arg is not exist
            Ok(env::var("CASE_INSENSITIVE").is_err())
        }
    }
}

/// run app
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

/// Search and case sensitive
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

/// Search but case insensitive
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // Convert to lowercase and return a new String
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
