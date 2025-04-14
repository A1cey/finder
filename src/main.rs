use error::Error;
use input::args;
use output::print_results;
use tokio_util::sync::CancellationToken;

mod drives;
mod error;
mod input;
mod output;
mod search;

#[tokio::main]
async fn main() {
    let token = CancellationToken::new();

    tokio::select! {
        () = run(token.clone()) => {},
        _ = tokio::signal::ctrl_c() => {
            token.cancel();
        }
    }
}

async fn run(cancel_token: CancellationToken) {
    match args() {
        Ok(args) => match search::search(&args, cancel_token).await {
            Ok(res) => {
                if let Some(res) = res {
                    print_results(&args.pattern, res, &args.case_sensitivity);
                }
            }
            Err(err) => Error::handle(&err),
        },
        Err(err) => Error::handle(&err),
    }
}
