use reqwest;

// tokio let's us use "async" on our main function
#[tokio::main]
async fn main() {
    let response = reqwest::get("https://api.spotify.com/v1/search")
        .await
        // each response is wrapped in a `Result` type
        // we'll unwrap here for simplicity
        .unwrap()
        .text()
        .await;
    println!("{:?}", response);
}
