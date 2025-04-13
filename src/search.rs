use std::path::Path;
use std::{path::PathBuf, sync::Arc};

use tokio::join;
use tokio::sync::mpsc::{self, Receiver, Sender, error::SendError};
use tokio::task::JoinHandle;
use tokio::fs::{ReadDir, DirEntry};

use crate::input::{Debug, OutputType, SearchType};
use crate::{error::Error, input::Args, output::print_match};

pub struct SearchResult {
    pub found: Vec<Arc<PathBuf>>,
    pub errors: Option<Vec<Error>>,
}

impl SearchResult {
    fn new(found: impl Into<Vec<Arc<PathBuf>>>, errors: Option<impl Into<Vec<Error>>>) -> Self {
        SearchResult {
            found: found.into(),
            errors: errors.map(std::convert::Into::into),
        }
    }
}

pub async fn search(args: &Args) -> Result<Option<SearchResult>, Error> {
    let (res_tx, res_rx) = mpsc::channel::<Result<Arc<PathBuf>, Error>>(100);
    let (process_tx, mut process_rx) = mpsc::channel::<JoinHandle<()>>(100);

    let match_path = create_match_path(&args.search_type);

    let result = match args.output_type {
        OutputType::NoStream => no_stream_processor(args.debug, res_rx),
        OutputType::Stream => stream_processor(args.pattern.clone(), args.debug, res_rx),
    };

    let runner = tokio::spawn(async move {
        while let Some(handle) = process_rx.recv().await {
            handle.await.ok();
        }
    });

    for path in args.selected_drives.clone() {
        next_dir(
            args.pattern.clone(),
            Arc::new(path),
            res_tx.clone(),
            process_tx.clone(),
            match_path.clone(),
        );
    }

    drop(res_tx);
    drop(process_tx);

    join!(runner, result).1.map_err(Into::into)
}

// this not async and dont await sender future -> works, but when shutdon tokio does not know about all handles -> poll errors
// when async then future cannot be sent between threads safely, cannot satisfy `impl Future<Output = ()>: Send
// with async_recursion macro: does not search system correctly -> error
fn next_dir(
    pattern: Arc<String>,
    path: Arc<PathBuf>,
    res_tx: Sender<Result<Arc<PathBuf>, Error>>,
    process_tx: Sender<JoinHandle<()>>,
    match_path: fn(&Path, &str) -> bool,
) {
    let handle = tokio::spawn(process_dir(
        pattern,
        path,
        res_tx,
        process_tx.clone(),
        match_path,
    ));
    
    let _ = process_tx.send(handle);
}

async fn process_dir(
    pattern: Arc<String>,
    path: Arc<PathBuf>,
    res_tx: Sender<Result<Arc<PathBuf>, Error>>,
    process_tx: Sender<JoinHandle<()>>,
    match_path: fn(&Path, &str) -> bool,
) {
    if !path.is_dir() {
        return;
    }

    let mut dir = match tokio::fs::read_dir(path.as_path()).await {
        Ok(dir) => dir,
        Err(err) => {
            let _ = res_tx.send(Err(Error::SearchIO(err, path))).await;
            return;
        }
    };

    read_dir(&mut dir, pattern, path, res_tx, process_tx, match_path).await;
}

async fn read_dir(
    dir: &mut ReadDir,
    pattern: Arc<String>,
    path: Arc<PathBuf>,
    res_tx: Sender<Result<Arc<PathBuf>, Error>>,
    process_tx: Sender<JoinHandle<()>>,
    match_path: fn(&Path, &str) -> bool
)
{
    while let Some(entry) = dir
        .next_entry()
        .await
        .transpose() 
    {
        if process_entry(
            entry,
            pattern.clone(),
            path.clone(),
            res_tx.clone(),
            process_tx.clone(),
            match_path.clone(),
        )
        .await
        .is_err() {
            return;
        }
    }
}

async fn process_entry(
    entry: Result<DirEntry, std::io::Error>,
    pattern: Arc<String>,
    path: Arc<PathBuf>,
    res_tx: Sender<Result<Arc<PathBuf>, Error>>,
    process_tx: Sender<JoinHandle<()>>,
    match_path: fn(&Path, &str) -> bool
)-> Result<(), SendError<Result<Arc<PathBuf>, Error>>> {
    match entry {
        Ok(entry) => {
            let path = Arc::new(entry.path());
            if match_path(&path, pattern.as_str()) {
                res_tx.send(Ok(path.clone())).await?;
            }
            
            next_dir(
                pattern.clone(),
                path.clone(),
                res_tx.clone(),
                process_tx,
                match_path.clone(),
            );
            
            Ok(())
        }
        Err(err) => {
            res_tx
                .send(Err(Error::SearchIO(err, path.clone())))
                .await
        }
    }
}

fn create_match_path(search_type: &SearchType) -> fn(&Path, &str) -> bool {
    match search_type {
        SearchType::Both => {
            |path: &Path, pattern: &str| path
                .to_str()
                .is_some_and( |name| name.contains(pattern))
        }
        SearchType::File => |path: &Path, pattern: &str| {
            path.is_file()
                && path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and( |name| name.contains(pattern))
        },
        SearchType::Dir => |path: &Path, pattern: &str| {
            path.is_dir()
                && path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and( |name| name.contains(pattern))
        },
    }
}

fn stream_processor(
    pattern: Arc<String>,
    debug: Debug,
    mut rx: Receiver<Result<Arc<PathBuf>, Error>>,
) -> JoinHandle<Option<SearchResult>> {
    tokio::spawn(async move {
        while let Some(res) = rx.recv().await {
            match res {
                Ok(path) => print_match(&pattern, &path),
                Err(err) => {
                    if debug == Debug::On {
                        Error::handle(&err);
                    }
                }
            }
        }

        None
    })
}

fn no_stream_processor(
    debug: Debug,
    mut rx: Receiver<Result<Arc<PathBuf>, Error>>,
) -> JoinHandle<Option<SearchResult>> {
    tokio::spawn(async move {
        let mut found = Vec::new();
        let mut errors = (debug == Debug::On).then_some(Vec::new());

        while let Some(res) = rx.recv().await {
            match res {
                Ok(path) => found.push(path),
                Err(err) => {
                    if let Some(errors) = &mut errors {
                        errors.push(err);
                    }
                }
            }
        }

        Some(SearchResult::new(found, errors))
    })
}