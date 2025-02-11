use std::{
    collections::{HashSet, VecDeque},
    path::{Path, PathBuf},
    sync::mpsc::{channel, Sender},
};

use crate::{drives::get_available_drive_names, error::Error};

pub struct SearchResult {
    pub found: Vec<PathBuf>,
    pub errors: Option<Vec<Error>>,
}

impl SearchResult {
    fn new(found: impl Into<Vec<PathBuf>>, errors: Option<impl Into<Vec<Error>>>) -> Self {
        SearchResult {
            found: found.into(),
            errors: errors.map(std::convert::Into::into),
        }
    }
}

pub async fn search(pattern: String, selected_drives: Option<HashSet<PathBuf>>, debug: bool) {
    let drives = if let Some(drives) = selected_drives {
        drives
    } else {
        match get_available_drive_names() {
            Ok(drives) => drives
                .into_iter()
                .map(|drive| Path::new(&format!("{drive}:\\")).into())
                .collect(),
            Err(err) => {
                Error::handle(&err);
                return;
            }
        }
    };

    let (tx, rx) = channel::<Result<PathBuf, Error>>();
    let mut tasks = Vec::new();

    let streamer = tokio::spawn(async move {
        while let Ok(res) = rx.recv() {
            match res {
                Ok(path) => println!("{}", path.display()),
                Err(err) => {
                    if debug {
                        Error::handle(&err);
                    }
                }
            }
        }
    });

    for path in drives {
        let tx = tx.clone();
        let pattern = pattern.clone();

        tasks.push(tokio::spawn(search_dir(path, pattern, tx)));
    }

    drop(tx);

    for task in tasks {
        if let Err(err) = task.await {
            Error::handle(&err.into());
        }
    }

    if let Err(err) = streamer.await {
        Error::handle(&err.into());
    }
}

pub async fn search_no_stream(
    pattern: String,
    selected_drives: Option<HashSet<PathBuf>>,
    debug: bool,
) -> Result<SearchResult, Error> {
    println!("Searching ...");

    let drives = selected_drives.unwrap_or(
        get_available_drive_names()?
            .into_iter()
            .map(|drive| Path::new(&format!("{drive}:\\")).into())
            .collect(),
    );

    let (tx, rx) = channel::<Result<PathBuf, Error>>();
    let mut tasks = Vec::new();

    let search_result = tokio::spawn(async move {
        let mut found = Vec::new();
        let mut errors = debug.then_some(Vec::new());

        while let Ok(res) = rx.recv() {
            match res {
                Ok(path) => found.push(path),
                Err(err) => {
                    if let Some(errors) = &mut errors {
                        errors.push(err);
                    }
                }
            }
        }

        SearchResult::new(found, errors)
    });

    for path in drives {
        let tx = tx.clone();
        let pattern = pattern.clone();

        tasks.push(tokio::spawn(search_dir(path, pattern, tx)));
    }

    drop(tx);

    for task in tasks {
        if let Err(err) = task.await {
            Error::handle(&err.into());
        }
    }

    search_result.await.map_err(std::convert::Into::into)
}

async fn search_dir(path: PathBuf, pattern: String, tx: Sender<Result<PathBuf, Error>>) {
    if !path.is_dir() {
        return;
    }

    let mut to_search = VecDeque::new();
    to_search.push_back(path);

    while let Some(path) = to_search.pop_front() {
        match tokio::fs::read_dir(&path).await {
            Ok(mut dir) => {
                while let Some(entry) = dir.next_entry().await.transpose() {
                    match entry {
                        Ok(entry) => {
                            let path = entry.path();
                            if path.to_str().map_or(false, |name| name.contains(&pattern)) {
                                if let Err(err) = tx.send(Ok(path.clone())) {
                                    Error::handle(&err.into());
                                    return;
                                }
                            };

                            match entry.metadata().await {
                                Ok(entry) => {
                                    if entry.is_dir() {
                                        to_search.push_back(path);
                                    }
                                }
                                Err(err) => {
                                    if let Err(err) = tx.send(Err(Error::IO(err, path))) {
                                        Error::handle(&err.into());
                                        return;
                                    }
                                }
                            }
                        }
                        Err(err) => {
                            if let Err(err) = tx.send(Err(Error::IO(err, path))) {
                                Error::handle(&err.into());
                                return;
                            }
                            break;
                        }
                    }
                }
            }
            Err(err) => {
                if let Err(err) = tx.send(Err(Error::IO(err, path))) {
                    Error::handle(&err.into());
                    return;
                }
            }
        }
    }
}
