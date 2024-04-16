mod client;
mod error;

use crate::model::core::request::Request;
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
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let args = Args::parse();
    dbg!(&args);

    let mut url = args.url;

    if !url.contains("://") {
        url.insert_str(0, "http://");
    }

    let url = Url::parse(url.as_str()).unwrap();
    dbg!(&url);

    let scheme = Scheme::from_str(url.scheme())?;
    let domain = url.domain().unwrap().to_owned();
    let method = HttpMethod::Get;
    let port = url.port().unwrap_or(scheme.default_port());
    let path = url.path().to_owned();
    let query = url.query().map(|s| s.to_owned());
    let headers = parse_headers(args.header)?;

    let request = Request {
        method,
        scheme,
        domain,
        port,
        path,
        query,
        headers,
    };
    dbg!(&request);

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
