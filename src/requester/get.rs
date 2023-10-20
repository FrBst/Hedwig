use hyper::{Client, Response, Error};

pub async fn get(uri: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();
    let uri = uri.parse()?;
    let resp = client.get(uri).await?;
    println!("Response: {}", resp.status());
    println!("end");


    Ok(())
}