#![deny(clippy::unwrap_used, clippy::expect_used)]

use std::{collections::VecDeque, error::Error, path::Path};

use input::Input;

mod drives;
mod input;
mod search;

fn main() {
    let args = std::env::args().collect::<VecDeque<_>>();

    println!("{args:#?}");

    match Input::get_args().and_then(|input| search::search(&input.pattern, input.selected_drives))
    {
        Ok(res) => handle_result(res),
        Err(err) => handle_error(err),
    };
}

fn handle_error(error: Box<dyn Error>) {
    println!("{error}");
}

fn handle_result(result: Vec<Box<Path>>) {
    result
        .into_iter()
        .for_each(|path| println!("{}", path.display()));
}
