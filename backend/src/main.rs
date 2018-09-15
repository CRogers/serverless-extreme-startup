extern crate aws_lambda as lambda;
#[macro_use] extern crate log;
extern crate rusoto_core;
extern crate rusoto_dynamodb;

use rusoto_core::Region;
use rusoto_dynamodb::{DescribeTableInput, DynamoDb, DynamoDbClient};

fn main() {
    lambda::logger::init();
    info!("New lambda started!");

    let client = DynamoDbClient::new(Region::EuWest2);
    
    lambda::gateway::start(move |_req| {
        match client.describe_table(DescribeTableInput { table_name: "test-table".to_string() }).sync() {
            Ok(output) => {
                println!("Output: {:?}", output);
            },
            Err(error) => {
                error!("Error: {:?}", error);
            }
        }

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