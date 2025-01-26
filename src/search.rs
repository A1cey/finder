use std::{
    collections::HashSet,
    error::Error,
    fmt::{Debug, Display},
    fs::read_dir,
    path::Path,
};

use crate::drives::get_available_drive_names;

pub(crate) fn search(
    pattern: &str,
    selected_drives: Option<HashSet<Box<Path>>>,
) -> Result<Vec<Box<Path>>, Box<dyn Error>> {
    println!("Searching ...");
    let mut results = Vec::new();

    let drives = selected_drives.unwrap_or(
        get_available_drive_names()?
            .into_iter()
            .map(|drive| Path::new(format!("{drive}:").as_str()).into())
            .collect(),
    );

    drives
        .iter()
        .try_for_each(|path| search_dir(path, pattern, &mut results))?;

    Ok(results)
}

fn search_dir<'a, T>(path: &Path, pattern: &str, results: &mut T) -> Result<(), Box<dyn Error>>
where
    T: Extend<Box<Path>>,
{
    match (*path).to_str() {
        Some(name) => name
            .contains(pattern)
            .then(|| results.extend(std::iter::once(path.into()))),
        None => Err(Box::new(SearchError::InvalidUnicode(
            path.display().to_string(),
        )))?,
    };

    if path.is_dir() {
        for entry in read_dir(path)? {
            let entry_path = entry?.path();
            search_dir(&entry_path, pattern, results)?;
        }
    }

    Ok(())
}

pub enum SearchError {
    InvalidUnicode(String),
}

impl Error for SearchError {}

impl Display for SearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Debug for SearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchError::InvalidUnicode(path) => write!(f, "Invalid Unicode in Path: {}", path),
        }
    }
}
