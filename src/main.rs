#![deny(clippy::unwrap_used, clippy::expect_used)]

use error::handle_error;
use input::Input;
use search::SearchResult;

mod drives;
mod error;
mod input;
mod search;

#[tokio::main]
async fn main() {
    if let Err(err) = match Input::get_args() {
        Ok(input) => search::search(input.pattern, input.selected_drives, input.debug).await,
        Err(err) => Err(err)
    }
    .map(|res| handle_result(res)) {
        handle_error(err);
    };
}

fn handle_result(result: SearchResult) {
    println!("Results:");
    result.found
        .into_iter()
        .for_each(|path| println!("{}", path.display()));
    
    if let Some(errors) = result.errors {
        println!("Errors:");
        errors
            .into_iter()
            .for_each(|err| handle_error(err));
    }
    
}
