use std::{env, fs};

pub fn run<'a>(config: Config) -> Result<Vec<String>, std::io::Error> {
    let perform_search = |contents: String| {
        let search = match config.ignore_case {
            true => search_case_insensitive,
            false => search_case_sensitive,
        };
        let search_result = search(&config.query, &contents);
        Ok::<Vec<String>, std::io::Error>(search_result)
    };
    fs::read_to_string(config.file_path).and_then(perform_search)
}

fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<String> {
    search(query, contents, None)
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<String> {
    search(query, contents, Some(|line: &str| line.to_lowercase()))
}

fn search<'a>(
    query: &str,
    contents: &'a str,
    line_transform: Option<fn(&str) -> String>,
) -> Vec<String> {
    let line_transform = line_transform.unwrap_or(|some_line: &str| some_line.to_string());
    let transformed_query = line_transform(query);
    contents
        .lines()
        .filter(|line| line_transform(line).contains(&transformed_query))
        .map(|line| line.to_string())
        .collect()
}

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
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn search_case_sensitive_test() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_sensitive(query, contents)
        );
    }

    #[test]
    fn search_case_insensitive_test() {
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
