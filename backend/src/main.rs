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
use tower_web::error::DefaultCatch;
use tower_web::middleware::Identity;
use std::net::SocketAddr;

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
        #[get("/test")]
        fn test(&self) -> Result<&'static str, ()> {
            self.describe_table();
            Ok("Hello ƛ!")
        }
    }
}

fn service_builder(region: Region) -> ServiceBuilder<Test, DefaultCatch, Identity> {
    lambda::logger::init();
    info!("New lambda started!");

    let client = DynamoDbClient::new(region);

    ServiceBuilder::new()
        .resource(Test { dynamodb: client })
}

fn main() {
    service_builder(Region::EuWest2)
        .run_lambda()
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let addr = SocketAddr::from(([127, 0, 0, 1], 9123));

        service_builder(Region::Custom {
            name: "local".to_owned(),
            endpoint: "http://localhost:8000".to_owned()
        })
            .run(&addr)
            .unwrap();
    }
}