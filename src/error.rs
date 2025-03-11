use std::{
    fmt::{Debug, Display},
    path::PathBuf,
    sync::Arc,
};

use tokio::io;

pub enum Error {
    Args(String),
    ChannelRecv(String),
    DrivesApi(u32),
    DrivesInvalidNumberOfDrives,
    IO(io::Error),
    SearchIO(io::Error, Arc<PathBuf>),
    TokioJoin(String),
}

impl Error {
    pub fn handle(error: &Error) {
        eprintln!("\x1b[31mErr\x1b[0m: {error}");
    }
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Args(err) => write!(f, "{err}"),
            Error::ChannelRecv(err) => write!(f, "Channel Receiver Error: {err}"),
            Error::DrivesApi(code) => write!(f, "Api Error: {code}"),
            Error::DrivesInvalidNumberOfDrives => write!(f, "Invalid Number of Drives."),
            Error::IO(err) => write!(f, "{}", err),
            Error::SearchIO(err, path) => write!(f, "{}: {}", path.display(), err),
            Error::TokioJoin(err) => write!(f, "Tokio Error: Join Error: {err}"),
        }
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(value: tokio::task::JoinError) -> Self {
        Self::TokioJoin(value.to_string())
    }
}

impl From<std::sync::mpsc::RecvError> for Error {
    fn from(value: std::sync::mpsc::RecvError) -> Self {
        Self::ChannelRecv(value.to_string())
    }
}

impl<T> From<std::sync::mpsc::SendError<T>> for Error {
    fn from(value: std::sync::mpsc::SendError<T>) -> Self {
        Self::ChannelRecv(value.to_string())
    }
}

impl From<clap::parser::MatchesError> for Error {
    fn from(value: clap::parser::MatchesError) -> Self {
        Self::Args(value.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}
