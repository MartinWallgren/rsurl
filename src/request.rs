use atty;
use atty::Stream;
use reqwest::header;

use reqwest::{self, Body, Client, Method, RequestBuilder};

use clap::ArgMatches;
use std::fs::File;
use std::io::{self, Read};

pub enum BodySource {
    StdIn,
    File(String),
}

impl BodySource {
    pub fn get_body(&self) -> Result<Body, Box<dyn std::error::Error>> {
        match &self {
            BodySource::File(path) => Ok(Body::from(File::open(&path)?)),
            BodySource::StdIn => BodySource::read_stdin(),
        }
    }

    fn read_stdin() -> Result<Body, Box<dyn std::error::Error>> {
        let mut payload = Vec::new();
        io::stdin().read_to_end(&mut payload)?;
        Ok(Body::from(payload))
    }
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

pub fn get_body_source(args: &ArgMatches) -> Option<BodySource> {
    if let Some(body) = args.value_of("body") {
        return Some(BodySource::File(body.to_string()));
    }

    println!("{:?}", atty::is(Stream::Stdin));
    if !atty::is(Stream::Stdin) {
        return Some(BodySource::StdIn);
    }
    None
}
