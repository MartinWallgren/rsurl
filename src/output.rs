use std::io;
use std::io::copy;

use reqwest::header::HeaderMap;

use reqwest::{Request, Response};

pub fn print_request(request: &Request) {
    println!("{} {}", request.method(), request.url());
    print_headers(&request.headers());
    println!();
}

pub fn print_response(mut response: Response) {
    print_headers(response.headers());

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    copy(&mut response, &mut stdout).expect("Failed to read response");
    println!();
}

fn print_headers(headers: &HeaderMap) {
    for (key, value) in headers.iter() {
        println!("{}: {}", key, String::from_utf8_lossy(value.as_bytes()));
    }
    println!();
}
