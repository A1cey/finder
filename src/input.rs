use std::{
    collections::{HashSet, VecDeque},
    path::{Path, PathBuf},
};

use crate::error::FinderError;

pub struct Input {
    pub pattern: String,
    pub selected_drives: Option<HashSet<PathBuf>>,
    pub debug: bool
}

impl Input {
    pub fn get_args() -> Result<Self, FinderError> {
        let args = std::env::args().skip(1).collect::<VecDeque<_>>();

        if args.is_empty() {
            Err(FinderError::IONoArgumentsProvided)?
        }

        let mut input = Self {
            pattern: String::new(),
            selected_drives: None,
            debug: false
        };

        let mut flag = Args::None;

        for mut arg in args {
            if arg.starts_with("-") {
                arg.remove(0);

                if arg == "-search" || arg == "s" {
                    flag = Args::Pattern
                } else if arg == "-path" || arg == "p" {
                    flag = Args::Drive
                } else if arg == "-debug" {
                    flag = Args::Debug
                } else {
                    Err(FinderError::IOInvalidArgumentSpecifier(arg))?
                }
            } else {
                match flag {
                    Args::None | Args::Pattern => {
                        if input.pattern.is_empty() {
                            input.pattern = arg
                        } else {
                            Err(FinderError::IOInvalidArgument(arg))?
                        }
                    }
                    Args::Drive => {
                        input
                            .selected_drives
                            .get_or_insert_default()
                            .insert(Path::new(&arg).into());
                    },
                    Args::Debug => {
                        input.debug = true;
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
    Debug
}
