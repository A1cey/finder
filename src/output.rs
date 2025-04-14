use std::path::Path;

use crate::{error::Error, input::CaseSensitivity, search::SearchResult};

pub fn print_results(pattern: &str, result: SearchResult, case_sensitivity: &CaseSensitivity) {
    println!("Results:");
    result
        .found
        .into_iter()
        .for_each(|path| print_match(pattern, &path, case_sensitivity));

    if let Some(errors) = result.errors {
        println!("Errors:");
        errors.into_iter().for_each(|err| Error::handle(&err));
    }
}

pub fn print_match(pattern: &str, path: &Path, case_sensitivity: &CaseSensitivity) {   
    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
        let colored_file_name = match case_sensitivity {
            CaseSensitivity::CaseSensitive => file_name.replace(pattern, &format!("\x1b[32m{}\x1b[0m", pattern)),
            CaseSensitivity::IgnoreCase => {               
                let lower_file_name = file_name.to_lowercase();
                let lower_pattern = pattern.to_lowercase();
                let mut colored = String::new();
                let mut last_match_end = 0;

                for (match_start, _) in lower_file_name.match_indices(&lower_pattern) {
                    colored.push_str(&file_name[last_match_end..match_start]);
                    colored.push_str(&format!("\x1b[32m{}\x1b[0m", &file_name[match_start..match_start + pattern.len()]));
                    last_match_end = match_start + pattern.len();
                }
                colored.push_str(&file_name[last_match_end..]);
                colored
            },
        };

        let parent_path = path.parent().map(|p| p.display().to_string()).unwrap_or_default();
        let full_path_string = if parent_path.is_empty() {
            colored_file_name
        } else {
            format!("{}\\{}", parent_path, colored_file_name)
        };
        println!("{}", full_path_string);
    } else {
        println!("{}", path.display());
    }
}
