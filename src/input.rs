use std::{
    collections::HashSet,
    env,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::drives::get_available_drive_names;

use super::error::Error;
use clap::{Arg, ArgAction, ArgMatches, Command, value_parser};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Debug {
    On,
    Off,
}

pub enum OutputType {
    Stream,
    NoStream,
}

pub enum SearchType {
    Dir,
    File,
    Both,
}

pub struct Args {
    pub pattern: Arc<String>,
    pub selected_drives: HashSet<PathBuf>,
    pub debug: Debug,
    pub output_type: OutputType,
    pub search_type: SearchType,
}

impl Args {
    fn new(
        pattern: Arc<String>,
        selected_drives: HashSet<PathBuf>,
        debug: Debug,
        output_type: OutputType,
        search_type: SearchType,
    ) -> Args {
        Args {
            pattern,
            selected_drives,
            debug,
            output_type,
            search_type,
        }
    }
}

pub fn args() -> Result<Args, Error> {
    let mut args = match_input_arguments();

    let pattern = match args.try_remove_one::<String>("pattern")? {
        Some(pat) => pat,
        None => args
            .try_remove_one::<String>("pattern_arg")?
            .expect("pattern or pattern_arg must be present"),
    };

    let mut selected_drives: HashSet<PathBuf> = args
        .try_remove_many::<PathBuf>("path")?
        .map_or_else(HashSet::new, std::iter::Iterator::collect);

    if args.get_flag("current_directory") {
        selected_drives.insert(env::current_dir()?);
    }

    if selected_drives.is_empty() {
        get_available_drive_names()?
            .into_iter()
            .map(|drive| Path::new(&format!("{drive}:\\")).into())
            .for_each(|path| {
                selected_drives.insert(path);
            });
    }

    let debug = if args.get_flag("debug") {
        Debug::On
    } else {
        Debug::Off
    };

    let output_type = if args.get_flag("no_stream") {
        OutputType::NoStream
    } else {
        OutputType::Stream
    };

    let only_dir = args.get_flag("dir");
    let only_file = args.get_flag("file");

    let search_type = if only_dir == only_file {
        SearchType::Both
    } else if only_dir {
        SearchType::Dir
    } else {
        SearchType::File
    };

    Ok(Args::new(
        Arc::new(pattern),
        selected_drives,
        debug,
        output_type,
        search_type,
    ))
}

fn match_input_arguments() -> ArgMatches {
    Command::new("finder_args")
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
            Arg::new("current_directory")
                .short('c')
                .long("current")
                .action(ArgAction::SetTrue)
                .help("The current directory is used as the root path for the search.")
        )
        .arg(
            Arg::new("dir")
                    .short('D')
                    .action(ArgAction::SetTrue)
                    .help("Only searches for directories.")
        )
        .arg(
            Arg::new("file")
                    .short('F')
                    .action(ArgAction::SetTrue)
                    .help("Only searches for files.")
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
        .get_matches()
}
