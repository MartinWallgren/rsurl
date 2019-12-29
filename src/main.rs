#![warn(clippy::all)]
pub mod output;
pub mod params;
pub mod request;

use crate::output::*;
use crate::request::*;

#[macro_use]
extern crate clap;
use clap::{App, AppSettings, Arg, ArgMatches};

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

fn get_args() -> ArgMatches<'static> {
    App::new("rsurl")
        .version(crate_version!())
        .author(crate_authors!())
        .setting(AppSettings::TrailingVarArg)
        .about("Http requests from the command line.")
        .arg(
            Arg::with_name("body")
                .short("b")
                .long("body")
                .value_name("FILE")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("method")
                .possible_values(&Method::variants())
                .help("the http method to use")
                .required(true),
        )
        .arg(Arg::with_name("url").help("the request url").required(true))
        .arg(
            Arg::with_name("item")
                .help("extra request items such as headers, query params etc")
                .multiple(true)
                .validator(params::validate)
                .required(false),
        )
        .get_matches()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = get_args();

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
