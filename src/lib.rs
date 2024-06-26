use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        return Ok(Config {
            query,
            file_path
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents: String = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &file_contents, true) {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a> (query: &'a str, file_contents: &'a str, is_case_sensitive: bool) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in file_contents.lines() {
        if is_case_sensitive {
            if line.contains(query) {
                results.push(line);
            }
        } else {
            if line.to_lowercase().contains(&query.to_lowercase()) {
                results.push(line);
            }
        }
    }
    
    return results;
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONTENTS: &str = "\
Rust:
safe, fast, productive.
Pick three.";

    #[test]
    fn case_insensitive() {
        let query = "Duct";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, CONTENTS, false)
        )
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, CONTENTS, true)
        )
    }
}