use std::{cmp, env, error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search_case_sensitive(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    search(query, contents, None)
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    search(query, contents, Some(|line: &str| line.to_lowercase()))
}

fn search<'a>(
    query: &str,
    contents: &'a str,
    line_transform: Option<fn(&str) -> String>,
) -> Vec<&'a str> {
    let line_transform = line_transform.unwrap_or(|some_line: &str| some_line.to_string());
    let mut search_result: Vec<&str> = Vec::new();
    let transformed_query = line_transform(query);
    for line in contents.lines() {
        let transformed_line = line_transform(line);
        if transformed_line.contains(&transformed_query) {
            search_result.push(line);
        }
    }
    search_result
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let arguments_required = 3;
        match args.len().cmp(&arguments_required) {
            cmp::Ordering::Less => Err("not enough arguments"),
            _ => Ok(Config {
                query: args[1].clone(),
                file_path: args[2].clone(),
                ignore_case: env::var("IGNORE_CASE").is_ok(),
            }),
        }
    }
}

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
            search_case_sensitive(query, contents)
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
}
