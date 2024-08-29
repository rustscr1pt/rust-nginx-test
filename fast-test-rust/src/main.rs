use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path!("hello")
        .map(|| "Hello, John!".to_string());
    println!("Running on 8000 port");
    warp::serve(hello)
        .run(([0, 0, 0, 0], 8000))
        .await;
}