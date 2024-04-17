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
}

fn main() -> Result<(), AppError> {
    let args = Args::parse();
    dbg!(&args);

    let resp = Client::new(args.url.as_str()).strict_url(!args.fix).send();
    dbg!(&resp);

    Ok(())
}

fn parse_headers(headers: Option<Vec<String>>) -> Result<RequestHeaders, AppError> {
    if headers.is_none() {
        return Ok(RequestHeaders::new());
    }
    let headers = headers.unwrap();

    let mut res = Vec::new();
    for h in headers {
        let mut parts = h.split(':');
        let key = parts.next();
        let value = parts.next();

        if let (Some(key), Some(value)) = (key, value) {
            res.push((key.to_owned(), value.to_owned()));
        } else {
            return Err(AppError::Header);
        }
    }
    Ok(res.into())
}
