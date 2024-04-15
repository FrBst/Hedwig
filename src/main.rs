use clap::{arg, Parser};
use http_body_util::BodyExt;
use model::http::http_method::HttpMethod;
use tokio::io;

use crate::model::core::{request::Request, scheme::Scheme};

mod model;
mod requester;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Name of the person to greet
    #[arg(short, long, default_value_t = HttpMethod::Get)]
    method: HttpMethod,

    // Number of times to greet
    #[arg(short, long, default_value_t = String::new())]
    url: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    dbg!(&args);

    let resp = &requester::get::send_request(Request {
        method: args.method,
        scheme: Scheme::Https,
        domain: args.url,
        port: 443,
        path: None,
        query: None,
        headers: None,
    }).await;

    dbg!(resp);
}
