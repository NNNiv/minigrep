use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Query string not specified"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("File not specified"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insesitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insesitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line.trim())
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensistive() {
        let query = "love";
        let contents = "\
        My girlfriend is so pretty.
        She is as pretty as a goddess.
        I love her so much.
        Love your girl, lads.";
        assert_eq!(vec!["I love her so much."], search(query, contents));
    }

    #[test]
    fn case_insensistive() {
        let query = "lOVe";
        let contents = "\
        My girlfriend is so pretty.
        She is as pretty as a goddess.
        I love her so much.";
        assert_eq!(
            vec!["I love her so much."],
            search_case_insesitive(query, contents)
        );
    }
}
