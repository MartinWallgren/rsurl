use reqwest;
use reqwest::header;

use reqwest::{Client, Method, RequestBuilder};

pub fn client() -> Client {
    Client::new()
}
pub fn builder(
    client: &Client,
    method: &str,
    url: &str,
) -> Result<RequestBuilder, Box<dyn std::error::Error>> {
    Ok(client.request(Method::from_bytes(method.as_bytes())?, url))
}

pub fn headers(request: RequestBuilder) -> RequestBuilder {
    default_headers(request)
}

fn default_headers(request: RequestBuilder) -> RequestBuilder {
    request
        .header(header::ACCEPT, "*/*")
        .header(header::ACCEPT_ENCODING, "gzip, deflate")
}
