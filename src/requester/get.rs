use std::error::Error;

use http_body_util::{BodyExt, Empty};
use hyper::{
    body::{Bytes, Incoming},
    client::conn::http1::handshake,
};
use hyper_tls::HttpsConnector;
use hyper_util::{
    client::legacy::Client,
    rt::{TokioExecutor, TokioIo},
};
use tokio::net::TcpStream;

use crate::model::{
    core::{request::Request, response::Response, scheme::Scheme},
    http::http_method::HttpMethod,
    request_headers::RequestHeaders,
};

pub async fn send_request(request: Request) -> Result<Response, Box<dyn Error>> {
    match request.scheme {
        Scheme::Http => send_http(request).await,
        Scheme::Https => send_https(request).await,
    }
}

async fn send_http(uri: Request) -> Result<Response, Box<dyn Error>> {
    let stream = TcpStream::connect(&uri.build_host()).await?;
    let io = TokioIo::new(stream);
    let (mut sender, conn) = handshake(io).await?;

    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    let req = hyper::Request::builder()
        .uri(uri.build_url())
        .header(hyper::header::HOST, uri.build_host().as_str())
        .body(Empty::<Bytes>::new())?;

    let mut resp = sender.send_request(req).await?;

    let response_body = extract_body(&mut resp).await?;

    Ok(Response::new(response_body, resp.status().into()))
}

async fn send_https(request: Request) -> Result<Response, Box<dyn Error>> {
    let https = HttpsConnector::new();
    let client = Client::builder(TokioExecutor::new()).build::<_, Empty<Bytes>>(https);

    let req = hyper::Request::builder()
        .method::<hyper::Method>(request.method.clone().into())
        .uri(request.build_url())
        .header(hyper::header::HOST, request.build_host().as_str())
        .body(Empty::<Bytes>::new())?;

    let mut resp = client.request(req).await?;

    let response_body = extract_body(&mut resp).await?;

    Ok(Response::new(response_body, resp.status().into()))
}

async fn extract_body(resp: &mut hyper::Response<Incoming>) -> Result<String, Box<dyn Error>> {
    let mut response_body = String::new();

    while let Some(next) = resp.frame().await {
        let frame = next?;
        if let Some(chunk) = frame.data_ref() {
            response_body.push_str(std::str::from_utf8(chunk.as_ref()).unwrap());
        }
    }

    Ok(response_body)
}

pub async fn test() {
    let future = send_request(crate::model::core::request::Request::new(
        HttpMethod::Get,
        Scheme::Https,
        "google.com".into(),
        443,
        "/".to_owned(),
        None,
        RequestHeaders::new(),
    ));

    dbg!(future.await.unwrap());
}

pub async fn test_http() {
    let future = send_request(crate::model::core::request::Request::new(
        HttpMethod::Get,
        Scheme::Http,
        "httpforever.com".into(),
        80,
        "/".to_owned(),
        None,
        RequestHeaders::new(),
    ));

    dbg!(future.await.unwrap());
}
