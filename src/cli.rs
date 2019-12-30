use clap::{App, AppSettings, Arg, ArgMatches};

use crate::params;

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

pub fn get_args() -> ArgMatches<'static> {
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
