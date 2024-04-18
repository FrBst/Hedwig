mod client;
mod error;

use crate::{client::Client, model::core::request::Request};
use std::str::FromStr;

use clap::Parser;
use error::AppError;
use model::{http::http_method::HttpMethod, request_headers::RequestHeaders};
use url::Url;

use crate::model::core::scheme::Scheme;

mod model;
mod requester;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    method: Option<HttpMethod>,

    #[arg(long)]
    url: String,

    #[arg(long, value_parser)]
    header: Option<Vec<String>>,

    #[arg(long, default_value_t = false)]
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
    dbg!(&resp);

    Ok(())
}
