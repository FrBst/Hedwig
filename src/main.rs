mod client;
mod error;
mod sender;

use crate::client::Client;

use clap::Parser;
use error::AppError;
use model::http::http_method::HttpMethod;

mod model;

/// CLI tool to send requests
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// HTTP method
    #[arg(long)]
    method: Option<HttpMethod>,

    /// target URL
    #[arg(long)]
    url: String,

    /// header in the format "key: value"
    #[arg(long, value_parser)]
    header: Option<Vec<String>>,

    /// guess some parts of the URL, for example the protocol
    #[arg(long)]
    fix: bool,

    /// follow redirects
    #[arg(long)]
    follow: bool,
}

fn main() -> Result<(), AppError> {
    let args = Args::parse();
    dbg!(&args);

    let mut client = Client::new(args.url.as_str())
        .strict_url(!args.fix)
        .follow(args.follow);

    if let Some(header) = args.header {
        client = client.headers(header)?
    }

    let resp = client.send();

    Ok(())
}
