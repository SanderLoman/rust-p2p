use reqwest::header::{HeaderMap, ACCEPT};
use reqwest::Client;
use serde_json::Value;

pub async fn get_metadata() -> Result<(String, String, String), Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    let res = client
        .get("http://127.0.0.1:5052/eth/v1/node/identity")
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    let v = serde_json::from_str::<Value>(&res)?;
    let seq = v["data"]["metadata"]["seq_number"]
        .as_str()
        .unwrap()
        .to_string();
    let attnets = v["data"]["metadata"]["attnets"]
        .as_str()
        .unwrap()
        .to_string();
    let syncnets = v["data"]["metadata"]["syncnets"]
        .as_str()
        .unwrap()
        .to_string();

    Ok((seq, attnets, syncnets))
}
