use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client,
};

use super::stroage::get_token;

pub fn get_client() -> Client {
    let mut headers = HeaderMap::new();
    let token = get_token();
    if let Some(token) = token {
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        );
    }
    reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap()
}
