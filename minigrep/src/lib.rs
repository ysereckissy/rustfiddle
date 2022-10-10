//! This file contains the implementation of the library used in the minigrep
//! console program. The main functions contained in this file are:
//! search used to search for a query string in the file
//! search_case_sensitive use to perform the same search but in case sensitive way.
//!
//! The following example code shows how to use the package
//! ```
//! use std::env;
//! use std::process;
//! use minigrep::Config;
//! use minigrep::mix;
//! use minigrep::PrimaryColor;

//! fn main() {
//!     let config = Config::build(env::args()).unwrap_or_else(|err| {
//!        eprintln!("Problem parsing arguments: {err}");
//!        process::exit(1);
//!    });
//!    if let Err(e) = minigrep::run(config) {
//!        eprintln!("Application error: {e}");
//!        process::exit(1);
//!    }

//!    let red = PrimaryColor::Red;
//!    let yellow = PrimaryColor::Yellow;
//!    mix(red, yellow);
//!}
//! ```

use std::error::Error;
use std::fs;
use std::env;

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }
    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;
    /// Combines two primary colors in equal amount to create a secondary color
    pub fn mix(_cl1: PrimaryColor, _cl2: PrimaryColor) -> SecondaryColor {
       SecondaryColor::Green
    }
}

/// Define a configuration that contains what's needed by minigrep to run.
///
/// # Example
///
///```rust
/// use minigrep::Config;
///
/// let config = Config {
///     query: String::from("player"),
///     file_path: String::from("search_file.txt"),
///     ignore_case: false,
/// };
///```
pub struct Config {
    /// The query string to search for in the file
    pub query: String,
    /// The path to the file that might contain the query string
    pub file_path: String,
    /// Search case sensitiveness
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // skip the first element which is the name of the program
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
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
        .map(|line| line.trim())
        .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .map(|line| line.trim())
        .collect()
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
        pick three.
        Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
        Rust:
        safe, fast, productive.
        pick three.
        Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}

