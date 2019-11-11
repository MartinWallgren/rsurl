#![warn(clippy::all)]
pub mod output;
pub mod request;

use crate::output::*;
use crate::request::*;

#[macro_use]
extern crate clap;
use clap::{App, AppSettings, Arg};

arg_enum! {
    #[derive(Debug)]
    pub enum Method {
        DELETE,
        GET,
        PATCH,
        POST,
        PUT,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = App::new("My Super Program")
        .version(crate_version!())
        .author(crate_authors!())
        .setting(AppSettings::AllowMissingPositional)
        .about("Http requests from the command line.")
        .arg(
            Arg::from_usage("<method> 'the http method to use'")
                .possible_values(&Method::variants())
                .required(true)
                .default_value("GET"),
        )
        .arg(
            Arg::with_name("url")
                .help("the request url")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    // unwrap safe due to url being mandatory
    let url = opts.value_of("url").unwrap();
    let method = opts.value_of("method").unwrap();

    let client = request::client();
    let mut rb = builder(&client, method, url).expect("Request failed");
    rb = headers(rb);
    rb = match get_body_source() {
        Some(s) => body(rb, s)?,
        None => rb,
    };

    let request = rb.build()?;
    print_request(&request);
    let response = client.execute(request)?;
    print_response(response);
    Ok(())
}
