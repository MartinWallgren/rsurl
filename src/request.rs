use reqwest;
use reqwest::{Method, Response};

pub fn request(method: &str, url: &str) -> Result<Response, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .request(Method::from_bytes(method.as_bytes())?, url)
        .send()?;
    Ok(response)
}
