extern crate aws_lambda as lambda;
#[macro_use] extern crate log;

use std::io::prelude::*;

fn main() {
    lambda::logger::init();
    info!("New lambda started!");

    lambda::gateway::start(|_req| {
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