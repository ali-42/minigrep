use std::{env, error::Error, fs};

pub struct Config {
    pub search: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let search = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("MINIGREP_CASE_INSENSITIVE").is_err();

        Ok(Config {
            search,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        research(&config.search, &content)
    } else {
        research_case_insensitive(&config.search, &content)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn research<'a>(search: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(search) {
            result.push(line);
        }
    }
    result
}

pub fn research_case_insensitive<'a>(search: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let search = search.to_lowercase();
    for line in content.lines() {
        if line.to_lowercase().contains(&search) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let search = "duct";
        let content = "\
Rust:
sécurité, rapidité, productivité.
Obtenez les trois en même temps.
Duct tape";
        assert_eq!(
            vec!["sécurité, rapidité, productivité."],
            research(search, content)
        );
    }

    #[test]
    fn case_insensitive() {
        let search = "rUsT";
        let content = "\
Rust:
sécurité, rapidité, productivité.
Obtenez les trois en même temps.
C'est pas rustique!";
        assert_eq!(
            vec!["Rust:", "C'est pas rustique!"],
            research_case_insensitive(search, content)
        );
    }
}
