#![warn(clippy::all)]
pub mod request;
#[macro_use]
extern crate clap;
use clap::{App, AppSettings, Arg};
use std::io;
use std::io::copy;

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

    let url = opts.value_of("url").unwrap();
    let method = opts.value_of("method").unwrap();
    // unwrap safe due to url being mandatory
    println!("{} {}", method, url);
    let mut response = request::request(method, url).expect("Request failed");
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    copy(&mut response, &mut stdout).expect("Failed to read response");
    Ok(())
}
