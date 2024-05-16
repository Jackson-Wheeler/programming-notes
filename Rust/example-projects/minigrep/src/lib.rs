use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, String> {
        // arg 1 - name of the program
        let program_name = match args.next() {
            Some(arg) => arg,
            None => return Err(format!("unable to find name of program...")),
        };

        let usage_message = 
            format!("Usage: {} <query> <file_path>\nSet environment variable IGNORE_CASE=1 to do case insesitive searching",
            program_name);

        // arg 2 - query
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err(format!("query argument not found\n{}", &usage_message)),
        };

        // arg 3 - file path
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err(format!("file path argument not found\n{}", &usage_message)),
        };

        // Environment variables
        // ignore case
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

// ORIGINAL SEARCH METHOD
// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     let mut results = Vec::new();

//     // loop through each line of contents
//     for line in contents.lines() {
//         // check if line contains the query string
//         if line.contains(query) {
//             results.push(line);
//         }
//     }

//     results
// }

// optimized search method
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

// optimized search method
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
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
