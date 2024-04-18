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

use crate::{
    error::AppError,
    model::{
        core::{request::Request, response::Response, scheme::Scheme},
        http::http_method::HttpMethod,
        request_headers::RequestHeaders,
    },
};

pub fn send_request(request: &Request) -> Result<Response, AppError> {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    match request.scheme {
        Scheme::Http => runtime.block_on(send_http(request)),
        Scheme::Https => runtime.block_on(send_https(request)),
    }
}

async fn send_http(request: &Request) -> Result<Response, AppError> {
    let stream = TcpStream::connect(&request.build_host())
        .await
        .map_err(|_| AppError::Request)?;
    let io = TokioIo::new(stream);
    let (mut sender, conn) = handshake(io).await.map_err(|_| AppError::Request)?;

    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    let req = hyper::Request::builder()
        .uri(request.build_url())
        .header(hyper::header::HOST, request.build_host().as_str())
        .body(Empty::<Bytes>::new())
        .map_err(|_| AppError::Request)?;

    let mut resp = sender
        .send_request(req)
        .await
        .map_err(|_| AppError::Request)?;

    let response_body = extract_body(&mut resp)
        .await
        .map_err(|_| AppError::Request)?;

    Ok(Response::new(
        response_body,
        resp.status().into(),
        resp.headers().to_owned(),
    ))
}

async fn send_https(request: &Request) -> Result<Response, AppError> {
    let https = HttpsConnector::new();
    let client = Client::builder(TokioExecutor::new()).build::<_, Empty<Bytes>>(https);

    let req = hyper::Request::builder()
        .method::<hyper::Method>(request.method.clone().into())
        .uri(request.build_url())
        .header(hyper::header::HOST, request.build_host().as_str())
        .body(Empty::<Bytes>::new())
        .map_err(|_| AppError::Request)?;

    let mut resp = client.request(req).await.map_err(|_| AppError::Request)?;

    let response_body = extract_body(&mut resp)
        .await
        .map_err(|_| AppError::Request)?;

    Ok(Response::new(
        response_body,
        resp.status().into(),
        resp.headers().to_owned(),
    ))
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
