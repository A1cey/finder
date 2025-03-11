use std::path::Path;

use crate::{error::Error, search::SearchResult};

pub fn print_results(pattern: &str, result: SearchResult) {
    println!("Results:");
    result
        .found
        .into_iter()
        .for_each(|path| print_match(pattern, &path));

    if let Some(errors) = result.errors {
        println!("Errors:");
        errors.into_iter().for_each(|err| Error::handle(&err));
    }
}

pub fn print_match(pattern: &str, path: &Path) {
    let s = path
        .display()
        .to_string()
        .replace(pattern, format!("\x1b[32m{}\x1b[0m", pattern).as_str());

    println!("{}", s);
}
