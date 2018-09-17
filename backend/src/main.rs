extern crate aws_lambda as lambda;
extern crate aws_lambda_tower_web;
#[macro_use] extern crate tower_web;
#[macro_use] extern crate log;
extern crate rusoto_core;
extern crate rusoto_dynamodb;

use rusoto_core::Region;
use rusoto_dynamodb::{DescribeTableInput, DynamoDb, DynamoDbClient};

use tower_web::ServiceBuilder;
use aws_lambda_tower_web::ServiceBuilderExt;

struct Test {
    dynamodb: DynamoDbClient,
}

impl Test {
    fn describe_table(&self) {
        match self.dynamodb.describe_table(DescribeTableInput { table_name: "test-table".to_string() }).sync() {
            Ok(output) => {
                println!("Output: {:?}", output);
            },
            Err(error) => {
                error!("Error: {:?}", error);
            }
        }
    }
}

impl_web! {
    impl Test {
        #[get("/")]
        fn test(&self) -> Result<&'static str, ()> {
            self.describe_table();
            Ok("Hello ƛ!")
        }
    }
}

fn main() {
    lambda::logger::init();
    info!("New lambda started!");

    let client = DynamoDbClient::new(Region::EuWest2);

    ServiceBuilder::new()
        .resource(Test { dynamodb: client })
        .run_lambda()
        .unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {

    }
}