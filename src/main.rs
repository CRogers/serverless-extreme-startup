extern crate aws_lambda as lambda;
#[macro_use] extern crate log;
extern crate reqwest;

use std::io::prelude::*;
use stopwatch::Stopwatch;

fn main() {
    lambda::logger::init();
    info!("New lambda started!");

    lambda::gateway::start(|_req| {
        let mut response_string = String::new();

        reqwest::get("https://www.googleapis.com/oauth2/v3/certs")
            .unwrap()
            .read_to_string(&mut response_string)
            .expect("To be able to read");

        let res = lambda::gateway::response()
            .status(200)
            .body("Hello ƛ!".into())?;

        Ok(res)
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {

    }
}