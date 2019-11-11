use std::io;
use std::io::copy;

use reqwest::{Request, Response};

pub fn print_request(request: &Request) {
    println!("{} {}", request.method(), request.url());
}

pub fn print_response(mut response: Response) {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    copy(&mut response, &mut stdout).expect("Failed to read response");
}
