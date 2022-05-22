use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool
}

impl Config {
  pub fn new(mut args: env::Args) -> Result<Self, &'static str> {
      args.next();

      let query = match args.next() {
          Some(arg) => arg,
          None => return Err("query argument is missing")
      };

      let filename = match args.next() {
        Some(arg) => arg,
        None => return Err("filename argument is missing")
      };

      // let case_sensitive = match env::var("MINIGREP_CASE_INSENSITIVE") {
      //   Ok(value) => value != "1",
      //   _ => true
      // };
      // Whatever the value:
      let case_sensitive = env::var("MINIGREP_CASE_INSENSITIVE").is_err();

      Ok(Config { query, filename, case_sensitive })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let content = fs::read_to_string(&config.filename)?;

  let results = if config.case_sensitive {
    search(&config.query, &content)
  } else {
    search_case_insensitive(&config.query, &content)
  };

  results.iter().for_each(|result| println!("{}", result));

  Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
  content
    .lines()
    .filter(|line| line.contains(query))
    .collect()
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();
  content
    .lines()
    .filter(|line| line.to_lowercase().contains(&query))
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }

  #[test]
  fn case_insensitive_search() {
    let query = "rUst";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
  }
}
