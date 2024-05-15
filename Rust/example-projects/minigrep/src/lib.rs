use std::env;
use std::error::Error;
use std::fs;

pub struct Config<'a> {
    pub query: &'a String,
    pub file_path: &'a String,
    pub ignore_case: bool,
}

impl<'a> Config<'a> {
    pub fn build(args: &[String]) -> Result<Config, String> {
        // If args is not equal to 3, then return usage information
        if args.len() != 3 {
            let err_msg = format!(
                "Invalid number of arguments\nUsage: {} <query> <file_path>\nSet environment variable IGNORE_CASE=1 to do case insesitive searching",
                args[0]
            );
            return Err(err_msg);
        }

        // parse command line arguments
        let query = args.get(1).expect("1st command line argument not found");
        let file_path = args.get(2).expect("2nd command line argument not found");

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        return Ok(Config {
            query,
            file_path,
            ignore_case,
        });
    }
}

// note: Box<dyn Error> means a type that implements the Error trait
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read file contents
    let contents = fs::read_to_string(config.file_path)?;

    // search contents for query
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    // output search results
    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    // loop through each line of contents
    for line in contents.lines() {
        // check if line contains the query string
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
