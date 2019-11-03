#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg};

arg_enum! {
    #[derive(Debug)]
    pub enum Method {
        POST,
        GET,
        PATCH,
        DELETE,
    }
}

fn main() {
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
    println!("{}", opts.value_of("url").unwrap());
}
