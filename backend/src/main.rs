extern crate aws_lambda as lambda;
#[macro_use] extern crate log;
extern crate rusoto_core;
extern crate rusoto_dynamodb;

use std::io::prelude::*;
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDbClient, ListTablesInput};

fn main() {
    lambda::logger::init();
    info!("New lambda started!");

    let client = DynamoDbClient::new(Region::EuWest2);
    match client.list_tables(Default::default()).sync() {
        Ok(output) => {
            match output.table_names {
                Some(table_names) => {
                    info!("Tables in database:");

                    for table_name in table_names {
                        info!("{}", table_name);
                    }
                },
                None => info!("No tables in database!"),
            }
        },
        Err(error) => {
            error!("Error: {:?}", error);
        }
    }

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