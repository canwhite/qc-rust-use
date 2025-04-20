// client.rs

use reqwest::Error;
use serde_json::{Value, to_string, from_str};


pub async fn fetch_data() -> Result<Value, Error> {
    // 发起 GET 请求
    let response = reqwest::get("https://jsonplaceholder.typicode.com/posts/1")
        .await?
        .json::<Value>()
        .await?;

    Ok(response)
}


pub fn _json_to_string(json: &Value) -> Result<String, serde_json::Error> {
    to_string(json)
}

pub fn _string_to_json(json_string: &str) -> Result<Value, serde_json::Error> {
    from_str(json_string)
}