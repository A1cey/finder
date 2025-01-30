use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    fmt::{Debug, Display},
    path::Path,
};

pub struct Input {
    pub pattern: String,
    pub selected_drives: Option<HashSet<Box<Path>>>,
}

impl Input {
    pub fn get_args() -> Result<Self, Box<dyn Error>> {
        let args = std::env::args().skip(1).collect::<VecDeque<_>>();

        if args.is_empty() {
            Err(IOError::NoArgumentsProvided)?
        }

        let mut input = Self {
            pattern: String::new(),
            selected_drives: None,
        };

        let mut flag = Args::None;

        for mut arg in args {
            if arg.starts_with("-") {
                arg.remove(0);

                if arg == "-search" || arg == "s" {
                    flag = Args::Pattern
                } else if arg == "-path" || arg == "p" {
                    flag = Args::Drive
                } else {
                    Err(IOError::InvalidArgumentSpecifier(arg))?
                }
            } else {
                match flag {
                    Args::None | Args::Pattern => {
                        if input.pattern.is_empty() {
                            input.pattern = arg
                        } else {
                            Err(IOError::InvalidArgument(arg))?
                        }
                    }
                    Args::Drive => {
                        input
                            .selected_drives
                            .get_or_insert_default()
                            .insert(Path::new(&arg).into());
                    }
                }
            }
        }

        Ok(input)
    }
}

enum Args {
    None,
    Pattern,
    Drive,
}

enum IOError {
    NoArgumentsProvided,
    InvalidArgumentSpecifier(String),
    InvalidArgument(String),
}

impl Error for IOError {}

impl Display for IOError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Debug for IOError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IOError::NoArgumentsProvided => write!(f, "No Arguments Provided"),
            IOError::InvalidArgumentSpecifier(arg) => {
                write!(f, "Invalid Argument Specifier: {arg}")
            }
            IOError::InvalidArgument(arg) => {
                write!(f, "Invalid Argument: {arg}")
            }
        }
    }
}
