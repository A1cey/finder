#![deny(clippy::unwrap_used)]

use error::Error;
use input::args;
use output::print_results;

mod drives;
mod error;
mod input;
mod output;
mod search;

#[tokio::main]
async fn main() {
    match args() {
        Ok(args) => match search::search(&args).await {
            Ok(res) => {
                if let Some(res) = res {
                    print_results(&args.pattern, res)
                }
            }
            Err(err) => Error::handle(&err),
        },
        Err(err) => Error::handle(&err),
    }
}
