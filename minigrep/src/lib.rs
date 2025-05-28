//! # MiniGrep
//!
//! A simple command-line tool for searching text files.
//!
//! This crate provides:
//! - A [`Config`] struct for parsing command line arguments.
//! - Functions like [`run`], [`search`], and [`search_case_insensitive`].
//! - Case-sensitive and case-insensitive search functionality based on env vars.
//!
//! # Usage
//!
//! Run the `run` function with a [`Config`] built from CLI args.
//!
//! # Example
//!
//! ```no_run
//! use minigrep::{run, Config};
//!
//! let args = vec![
//!     "minigrep".to_string(),
//!     "search_term".to_string(),
//!     "file.txt".to_string(),
//! ];
//! let config = Config::build(&args).unwrap();
//! run(config).unwrap();
//! ```
//!
//! [`Config`]: struct.Config.html
//! [`run`]: fn.run.html
//! [`search`]: fn.search.html
//! [`search_case_insensitive`]: fn.search_case_insensitive.html

#![deny(missing_docs)]

use std::env;
use std::error::Error;
use std::fs::{self};

/// Configuration for searching within a file.
///
/// This struct holds the parameters needed to perform a search,
/// including the search `query`, the path to the `file_path`, and
/// whether the search should be `ignore_case`.
///
/// It is typically constructed using [`Config::build`].
#[derive(Debug)]
pub struct Config {
    /// The string to search for.
    pub query: String,
    /// The path to the file where the search will be performed.
    pub file_path: String,
    /// If `true`, the search ignores content cases.
    pub ignore_case: bool,
}

impl Config {
    /// Constructs a new [`Config`] from a slice of command-line arguments.
    ///
    /// # Arguments
    ///
    /// * `args` - A slice of `String` values, typically `std::env::args().collect()`.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Config)` if the required arguments are present, otherwise returns
    /// an `Err` with a static string message indicating insufficient arguments.
    ///
    /// The `ignore_case` field is determined by checking whether the environment
    /// variable `IGNORE_CASE` is set (regardless of value).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::env;
    /// use minigrep::Config;
    ///
    ///
    /// env::set_var("IGNORE_CASE", "1")
    ///
    /// let args = vec![
    ///     "program_name".to_string(),
    ///     "search_term".to_string(),
    ///     "file.txt".to_string(),
    /// ];
    ///
    /// let config = Config::build(&args).unwrap();
    /// assert_eq!(config.query, "search_term");
    /// assert_eq!(config.file_path, "file.txt");
    /// assert!(config.ignore_case);
    /// ```
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case: bool = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case: ignore_case,
        })
    }
}

/// Runs the search using the given [`Config`] object.
///
/// Depending on `config.ignore_case`, it uses either [`search`] or [`search_case_insensitive`].
///
/// # Errors
///
/// Returns an error if the file at `config.file_path` cannot be read.
///
/// # Example
///
/// ```no_run
/// use minigrep::{run, Config};
///
/// let config = Config {
///     query: "rust".to_string(),
///     file_path: "test.txt".to_string(),
///     ignore_case: true,
/// };
///
/// // This will print results to stdout
/// run(config).unwrap();
/// ```
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
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

/// Search with a query for `case_sensitive` contents.
///
/// # Example
///
/// ```rust
/// use minigrep::search;
///
/// let query = "duct";
/// let contents = "/
/// Rust:
/// safe, fast, productive.
/// Pick three.";
///
/// assert_eq!(vec!["safe, fast, productive."], search(query, contents))
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

/// Search with a query for `case_insensitive` contents.
///
/// # Example
///
/// ```rust
/// use minigrep::search_case_insensitive;
///
/// let query = "rUsT";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.
/// Trust me.";
///
/// assert_eq!(
///     vec!["Rust:", "Trust me."],
///     search_case_insensitive(query, contents)
/// );
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "/
        Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
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
        )
    }
}
