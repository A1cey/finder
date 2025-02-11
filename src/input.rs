use std::{collections::HashSet, path::PathBuf};

use super::error::Error;
use clap::{value_parser, Arg, ArgAction, Command};

pub struct Args {
    pub pattern: String,
    pub selected_drives: Option<HashSet<PathBuf>>,
    pub debug: bool,
    pub no_stream: bool,
}

impl Args {
    fn new(
        pattern: String,
        selected_drives: Option<HashSet<PathBuf>>,
        debug: bool,
        no_stream: bool,
    ) -> Args {
        Args {
            pattern,
            selected_drives,
            debug,
            no_stream,
        }
    }
}

pub fn args() -> Result<Args, Error> {
    let mut args = Command::new("finder_args")
        .version(env!("CARGO_PKG_VERSION"))
        .name(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("pattern")
                .value_name("PATTERN")
                .conflicts_with("pattern_arg")
                .required(true)
                .help("The pattern to search for. Provide either this positional argument OR the --search flag, but not both.")
                .num_args(1),
        )
        .arg(
            Arg::new("pattern_arg")
                .value_name("PATTERN")
                .short('s')
                .long("search")
                .help("The pattern to search for (alternative). Provide either this --search flag OR the positional argument, but not both.")
                .num_args(1),
        )
        .arg(
            Arg::new("path")
                .value_name("PATH")
                .short('p')
                .long("path")
                .help("The root path(s) for the search separated by spaces.")
                .num_args(0..)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("debug")
                .long("debug")
                .action(ArgAction::SetTrue)
                .help("Print all errors to the console."),
        )
        .arg(
            Arg::new("no_stream")
                .long("no-stream")
                .action(ArgAction::SetTrue)
                .help("The result of the search will be only returned at the end as one block.\n\
                    This can have the effect, that all existing results were found\n\
                    but they are not displayed because some paths are still searched.",
                ),
        )
        .disable_help_flag(true)
        .arg(Arg::new("help")
            .short('h')
            .long("help")
            .help("Print help info.")
            .action(ArgAction::Help)
        )
        .disable_version_flag(true)
        .arg(Arg::new("version")
            .short('v')
            .long("version")
            .help("Print the version.")
            .action(ArgAction::Version)
        )
        .get_matches();

    let pattern = match args.try_remove_one::<String>("pattern")? {
        Some(pat) => pat,
        None => args
            .try_remove_one::<String>("pattern_arg")?
            .expect("pattern or pattern_arg must be present"),
    };

    let selected_drives = args
        .try_remove_many::<PathBuf>("path")?
        .map(std::iter::Iterator::collect);

    let debug = args.get_flag("debug");
    let no_stream = args.get_flag("no_stream");

    Ok(Args::new(pattern, selected_drives, debug, no_stream))
}
