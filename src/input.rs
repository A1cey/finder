use std::{
    collections::{HashSet, VecDeque},
    path::{Path, PathBuf},
};

use crate::error::Error;

enum Args {
    None,
    Pattern,
    Drive,
}

pub struct Input {
    pub pattern: String,
    pub selected_drives: Option<HashSet<PathBuf>>,
    pub debug: bool,
    pub no_stream: bool,
}

impl Input {
    pub fn get_args() -> Result<Self, Error> {
        let args = std::env::args().skip(1).collect::<VecDeque<_>>();

        if args.is_empty() {
            Err(Error::IONoArgumentsProvided)?;
        }

        let mut input = Self {
            pattern: String::new(),
            selected_drives: None,
            debug: false,
            no_stream: false,
        };

        let mut flag_with_arg = Args::None;

        for mut arg in args {
            if arg.starts_with('-') {
                arg.remove(0);

                if arg == "-search" || arg == "s" {
                    flag_with_arg = Args::Pattern;
                } else if arg == "-path" || arg == "p" {
                    flag_with_arg = Args::Drive;
                } else if arg == "-debug" {
                    input.debug = true;
                } else if arg == "-no-stream" {
                    input.no_stream = true;
                } else {
                    Err(Error::IOInvalidArgumentSpecifier(arg))?;
                }
            } else {
                match flag_with_arg {
                    Args::None | Args::Pattern => {
                        if input.pattern.is_empty() {
                            input.pattern = arg;
                        } else {
                            Err(Error::IOInvalidArgument(arg))?;
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
