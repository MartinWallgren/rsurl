use atty;
use atty::Stream;
use reqwest::header;

use reqwest::{self, Body, Client, Method, RequestBuilder};

use std::fs::File;
use std::io::{self, Read};
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

pub fn body(request: RequestBuilder, body: Option<Body>) -> RequestBuilder {
    match body {
        Some(b) => request.body(b),
        None => request,
    }
}

fn read_stdin() -> Result<Body, Box<dyn std::error::Error>> {
    let mut payload = Vec::new();
    io::stdin().read_to_end(&mut payload)?;
    Ok(Body::from(payload))
}

pub fn get_body(in_file: Option<&str>) -> Result<Option<Body>, Box<dyn std::error::Error>> {
    if in_file.is_some() {
        return Ok(Some(Body::from(File::open(in_file.unwrap())?)));
    }
    if !atty::is(Stream::Stdin) {
        //TODO: Add a way to not read body for std_in for non terminal.
        return Ok(Some(read_stdin()?));
    }
    Ok(None)
}
