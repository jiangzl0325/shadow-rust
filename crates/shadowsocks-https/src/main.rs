use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use reqwest::{Url};
use tokio;

#[tokio::main]
async fn main() {
    let data = request_data().await.unwrap();
    println!("{:?}", data);
}

async fn request_data() -> Result<String, Box<dyn Error>> {
    let mut map = HashMap::new();
    map.insert("q", "设置");
    map.insert("page", "1");
    map.insert("pageSize", "10");
    map.insert("ctoken", "69_e6Yu_Zr_G8bLWy1KQNxaK");
    println!("map: {:?}", map);
    // 设置cookie
    let cookie = "ctoken=69_e6Yu_Zr_G8bLWy1KQNxaK";
    let url = "https://www.baidu.com".parse::<Url>().unwrap();

    let client = reqwest::Client::builder()
        .build()
        .unwrap();

    let res = client
        .get("https://www.baidu.com")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}
