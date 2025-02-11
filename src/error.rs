use std::{
    fmt::{Debug, Display},
    path::PathBuf,
};

use tokio::io;

pub enum Error {
    Args(String),
    ChannelRecv(String),
    DrivesApi(u32),
    DrivesInvalidNumberOfDrives,
    IO(io::Error, PathBuf),
    TokioJoin(String),
}

impl Error {
    pub fn handle(error: &Error) {
        eprintln!("{error}");
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
            Error::IO(err, path) => write!(f, "{}: {}", path.display(), err),
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

impl From<std::sync::mpsc::SendError<Result<PathBuf, Error>>> for Error {
    fn from(value: std::sync::mpsc::SendError<Result<PathBuf, Error>>) -> Self {
        Self::ChannelRecv(value.to_string())
    }
}

impl From<clap::parser::MatchesError> for Error {
    fn from(value: clap::parser::MatchesError) -> Self {
        Self::Args(value.to_string())
    }
}
