use reqwest;
use serde::{Deserialize, Serialize};
use tokio;

#[derive(Serialize, Deserialize, Debug)]
struct Comment {
    postId: u32,
    id: u32,
    name: String,
    email: String,
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = "https://jsonplaceholder.typicode.com/posts/1/comments";
    let body = reqwest::get(url).await?.text().await?;

    let comments: Vec<Comment> = serde_json::from_str(&body).unwrap();
    println!("{comments:#?}");

    Ok(())
}
