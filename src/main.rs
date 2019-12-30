#![warn(clippy::all)]
pub mod cli;
pub mod output;
pub mod params;
pub mod request;

use crate::output::*;
use crate::request::*;

#[macro_use]
extern crate clap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::get_args();

    // unwrap safe for mandatory values
    let url = args.value_of("url").unwrap();
    let method = args.value_of("method").unwrap();

    let client = request::client();
    let mut rb = builder(&client, method, url).expect("Request failed");

    let params = match args.values_of("item") {
        Some(items) => params::parse(items.collect()).unwrap(),
        None => vec![],
    };
    rb = headers(rb, &params);

    if let Some(bs) = get_body_source(&args) {
        rb = rb.body(bs.get_body()?);
    }

    let request = rb.build()?;
    print_request(&request);
    let response = client.execute(request)?;
    print_response(response);
    Ok(())
}
