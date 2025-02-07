use std::{
    error::Error,
    fmt::{Debug, Display}, path::PathBuf,
};

pub enum FinderError {
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
    TokioJoin(String)
}

impl Error for FinderError {}

impl Display for FinderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Debug for FinderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FinderError::ChannelRecv(err) => write!(f, "Channel Receiver Error: {err}"),
            FinderError::DrivesApi(code) => write!(f, "Api Error: {code}"),
            FinderError::DrivesInvalidNumberOfDrives => write!(f, "Invalid Number of Drives."),
            FinderError::IOIsADirectory => write!(f, "IO Error: Is a directory."),
            FinderError::IONotADirectory => write!(f, "IO Error: Is not a directory."),
            FinderError::IONoArgumentsProvided => write!(f, "No Arguments Provided"),
            FinderError::IOInvalidArgumentSpecifier(arg) => {
                write!(f, "Invalid Argument Specifier: {arg}")
            }
            FinderError::IOInvalidArgument(arg) => {
                write!(f, "Invalid Argument: {arg}")
            }
            FinderError::IONotFound => write!(f, "IO Error: Not found."),
            FinderError::IOOther(err) => write!(f, "IO Error: {err}"),
            FinderError::IOPermissionDenied => write!(f, "Permission denied."),
            FinderError::TokioJoin(err) => write!(f,"Tokio Error: Join Error: {err}")
        }
    }
}

impl From<std::io::Error> for FinderError {
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

impl From<tokio::task::JoinError> for FinderError {
    fn from(value: tokio::task::JoinError) -> Self {
        Self::TokioJoin(value.to_string())
    }
}

impl From<std::sync::mpsc::RecvError> for FinderError {
    fn from(value: std::sync::mpsc::RecvError) -> Self {
        Self::ChannelRecv(value.to_string())
    }
}

impl From<std::sync::mpsc::SendError<Result<PathBuf, FinderError>>> for FinderError {
    fn from(value: std::sync::mpsc::SendError<Result<PathBuf, FinderError>>) -> Self {
        Self::ChannelRecv(value.to_string())
    }
}

pub fn handle_error(error: FinderError) {
    eprintln!("{error}");
}