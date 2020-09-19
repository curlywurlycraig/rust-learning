use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub is_case_insensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let is_case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, is_case_insensitive: !is_case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(config.filename)?;

    let results = if config.is_case_insensitive {
        search_case_insensitive(&config.query, &file_contents)
    } else {
        search(&config.query, &file_contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "pro";

        let contents = "\
Rust:
safe, fast, productive.
Pick three";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn two_results() {
        let query = "e";
        let contents = "\
Rust:
safe, fast, productive.
Pick three";

        assert_eq!(vec!["safe, fast, productive.", "Pick three"], search(query, contents));
    }

    #[test]
    fn no_results() {
        let query = "unicorn";
        let contents = "\
Rust:
safe, fast, productive.
Pick three";

        let expected_result: Vec<&str> = Vec::new();

        assert_eq!(expected_result, search(query, contents));
    }
}