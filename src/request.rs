use atty;
use atty::Stream;
use reqwest::header;

use reqwest::{self, Body, Client, Method, RequestBuilder};

use std::io::{self, Read};
pub enum BodySource {
    StdIn,
}

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

pub fn body(
    request: RequestBuilder,
    source: BodySource,
) -> Result<RequestBuilder, Box<dyn std::error::Error>> {
    let rb = match source {
        BodySource::StdIn => request.body(read_stdin()?),
    };
    Ok(rb)
}

fn read_stdin() -> Result<Body, Box<dyn std::error::Error>> {
    let mut payload = Vec::new();
    io::stdin().read_to_end(&mut payload)?;
    Ok(Body::from(payload))
}

pub fn get_body_source() -> Option<BodySource> {
    println!("{:?}", atty::is(Stream::Stdin));
    if !atty::is(Stream::Stdin) {
        //TODO: Add a way to not read body for std_in for non terminal.
        return Some(BodySource::StdIn);
    }
    None
}
