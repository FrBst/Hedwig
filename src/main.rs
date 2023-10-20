use std::future::Future;
use futures::executor::block_on;

mod requester;

#[tokio::main]
async fn main() {
    // let args = Args::parse();

    let future = requester::get::get("http://api.genderize.io?name=mary");
    match future.await {
        Ok(_) => { println!("Works"); },
        Err(err) => { dbg!(err); }
    }
}