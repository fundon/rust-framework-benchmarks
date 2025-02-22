#![deny(warnings)]
use warp::Filter;

#[tokio::main]
async fn main() {
    // Match any request and return hello world!
    let routes = warp::any().map(|| "Hello world!");

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}
