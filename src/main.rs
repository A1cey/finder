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
    let args = Input::get_args();
   
    if args.is_err() {
        let err = unsafe { args.unwrap_err_unchecked() };
        handle_error(err);
        return;
    }
    
    let args = args.unwrap();
    
    if args.no_stream {
        match search::search_no_stream(args.pattern, args.selected_drives, args.debug).await {
            Ok(result) => handle_result(result),
            Err(err) => handle_error(err),
        }
    } else {
        search::search(args.pattern, args.selected_drives, args.debug).await
    };
}

fn handle_result(result: SearchResult) {
    println!("Results:");
    result
        .found
        .into_iter()
        .for_each(|path| println!("{}", path.display()));

    if let Some(errors) = result.errors {
        println!("Errors:");
        errors.into_iter().for_each(|err| handle_error(err));
    }
}
