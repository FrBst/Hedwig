mod client;
mod error;
mod sender;

use crate::client::Client;

use clap::Parser;
use error::AppError;
use model::http::http_method::HttpMethod;

mod model;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    method: Option<HttpMethod>,

    #[arg(long)]
    url: String,

    #[arg(long, value_parser)]
    header: Option<Vec<String>>,

    #[arg(long)]
    fix: bool,

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
