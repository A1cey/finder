#![deny(clippy::unwrap_used, clippy::expect_used)]

use std::{error::Error, path::PathBuf};

use input::Input;

mod drives;
mod input;
mod search;

fn main() {
    match Input::get_args().and_then(|input| search::search(&input.pattern, input.selected_drives))
    {
        Ok(res) => handle_result(res),
        Err(err) => handle_error(err),
    };
}

fn handle_error(error: Box<dyn Error>) {
    eprintln!("{error}");
}

fn handle_result(result: Vec<PathBuf>) {
    result
        .into_iter()
        .for_each(|path| eprintln!("{}", path.display()));
}
