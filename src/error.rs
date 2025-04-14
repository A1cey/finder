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
    TokioSend(String),
}

impl Error {
    pub fn handle(error: &Self) {
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
           Self::Args(err) => write!(f, "{err}"),
           Self::ChannelRecv(err) => write!(f, "Channel Receiver Error: {err}"),
           Self::DrivesApi(code) => write!(f, "Api Error: {code}"),
           Self::DrivesInvalidNumberOfDrives => write!(f, "Invalid Number of Drives."),
           Self::IO(err) => write!(f, "{err}"),
           Self::SearchIO(err, path) => write!(f, "{}: {}", path.display(), err),
           Self::TokioJoin(err) => write!(f, "Tokio Error: Join Error: {err}"),
           Self::TokioSend(err) => write!(f, "Tokio Error: Send Error: {err}"),
        }
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(value: tokio::task::JoinError) -> Self {
        Self::TokioJoin(value.to_string())
    }
}

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for Error {
    fn from(value: tokio::sync::mpsc::error::SendError<T>) -> Self {
        Self::TokioSend(value.to_string())
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
