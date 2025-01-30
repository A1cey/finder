use std::{
    collections::HashSet,
    error::Error,
    fs::read_dir,
    path::{Path, PathBuf},
};

use crate::drives::get_available_drive_names;

pub fn search(
    pattern: &str,
    selected_drives: Option<HashSet<Box<Path>>>,
) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    println!("Searching ...");
    let mut results = Vec::new();

    let drives = selected_drives.unwrap_or(
        get_available_drive_names()?
            .into_iter()
            .map(|drive| Path::new(&format!("{drive}:\\")).into())
            .collect(),
    );

    for path in drives {
        if let Err(err) = search_dir(&path, pattern, &mut results) {
            eprintln!("Error searching '{}': {}", path.display(), err);
        }
    }

    Ok(results)
}

fn search_dir<T>(path: &Path, pattern: &str, results: &mut T) -> Result<(), Box<dyn Error>>
where
    T: Extend<PathBuf>,
{
    if path.to_str().map_or(false, |name| name.contains(pattern)) {
        results.extend(std::iter::once(path.into()));
    };

    if path.is_dir() {
        for entry in read_dir(path)? {
            let entry_path = entry?.path();
            if let Err(err) = search_dir(&entry_path, pattern, results) {
                eprintln!("Error searching '{}': {}", path.display(), err);
            }
        }
    }

    Ok(())
}
