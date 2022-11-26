use std::{collections::HashMap, env, error::Error, fs};

pub struct Config {
    pub file_path: String,
    pub query: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();

        let (query_key, file_key, ignore_case_key) = ("query", "file", "ignore_case");

        let mut map = HashMap::new();
        for arg in args {
            let arg = arg.split('=').collect::<Vec<_>>();
            if arg.len() != 2 {
                return Err("Invalid argument.");
            }

            let (k, v) = (arg[0], arg[1]);

            if !vec![query_key, file_key, ignore_case_key].contains(&k) {
                return Err("Invalid argument.");
            }

            if map.contains_key(k) {
                return Err("Duplicate argument.");
            }

            map.insert(k.to_string(), v.to_string());
        }

        let file_path = map
            .get(file_key)
            .expect("No file path provided")
            .to_string();
        let query = map.get(query_key).expect("No query provided").to_string();
        let ignore_case = map.contains_key(ignore_case_key) || env::var("IGNORE_CASE").is_ok();

        Ok(Self {
            file_path,
            query,
            ignore_case,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &text)
    } else {
        search(&config.query, &text)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn search_insensitive() {
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
