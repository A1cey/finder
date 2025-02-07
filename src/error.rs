use std::{
    fmt::{Debug, Display},
    path::PathBuf,
};

pub enum Error {
    ChannelRecv(String),
    DrivesApi(u32),
    DrivesInvalidNumberOfDrives,
    IOIsADirectory,
    IONotADirectory,
    IONoArgumentsProvided,
    IOInvalidArgumentSpecifier(String),
    IOInvalidArgument(String),
    IONotFound,
    IOOther(String),
    IOPermissionDenied,
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
            Error::ChannelRecv(err) => write!(f, "Channel Receiver Error: {err}"),
            Error::DrivesApi(code) => write!(f, "Api Error: {code}"),
            Error::DrivesInvalidNumberOfDrives => write!(f, "Invalid Number of Drives."),
            Error::IOIsADirectory => write!(f, "IO Error: Is a directory."),
            Error::IONotADirectory => write!(f, "IO Error: Is not a directory."),
            Error::IONoArgumentsProvided => write!(f, "No Arguments Provided"),
            Error::IOInvalidArgumentSpecifier(arg) => {
                write!(f, "Invalid Argument Specifier: {arg}")
            }
            Error::IOInvalidArgument(arg) => {
                write!(f, "Invalid Argument: {arg}")
            }
            Error::IONotFound => write!(f, "IO Error: Not found."),
            Error::IOOther(err) => write!(f, "IO Error: {err}"),
            Error::IOPermissionDenied => write!(f, "Permission denied."),
            Error::TokioJoin(err) => write!(f, "Tokio Error: Join Error: {err}"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        match value.kind() {
            std::io::ErrorKind::NotFound => Self::IONotFound,
            std::io::ErrorKind::PermissionDenied => Self::IOPermissionDenied,
            std::io::ErrorKind::NotADirectory => Self::IONotADirectory,
            std::io::ErrorKind::IsADirectory => Self::IOIsADirectory,
            _ => Self::IOOther(value.to_string()),
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
