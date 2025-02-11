#![deny(clippy::unwrap_used, clippy::expect_used)]

use error::Error;
use input::args;
use search::SearchResult;

mod drives;
mod error;
mod input;
mod search;

#[tokio::main]
async fn main() {
    match args() {
        Ok(args) => {
            if args.no_stream {
                match search::search_no_stream(args.pattern, args.selected_drives, args.debug).await
                {
                    Ok(result) => handle_result(result),
                    Err(err) => Error::handle(&err),
                }
            } else {
                search::search(args.pattern, args.selected_drives, args.debug).await;
            };
        }
        Err(err) => Error::handle(&err),
    }
}

fn handle_result(result: SearchResult) {
    println!("Results:");
    result
        .found
        .into_iter()
        .for_each(|path| println!("{}", path.display()));

    if let Some(errors) = result.errors {
        println!("Errors:");
        errors.into_iter().for_each(|err| Error::handle(&err));
    }
}
