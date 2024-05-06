use std::fs;
use std::error::Error;

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

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
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

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
    help: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.contains(&String::from("--help")) {
            return Ok(Config {
                query: "".to_string(),
                file_path: "".to_string(),
                ignore_case: false,
                help: true
            });
        }
        

        if args.len() < 3{
            return Err("Expected more arguments!");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = args.contains(&String::from("--ignore_case"));


        Ok(Config {
            query,
            file_path,
            ignore_case,
            help: false
        })
    }
}

pub fn print_help() {
        println!("\
Usage: grepzilla PATTERN [FILE]... [OPTION]...
Search for PATTERN in FILE.
Example: grepzilla 'hello world' main.rs --ignore_case

Pattern and selection:
    --ignore_case   ignore case distinction in pattern and data

Miscellaneous:
    --help          display this help message and exit");
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if config.help {
        print_help();
        return Ok(());
    }

    let contents = fs::read_to_string(config.file_path)?;

    let results = match config.ignore_case {
        true => search_case_insensitive(&config.query, &contents),
        false => search(&config.query, &contents),
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

