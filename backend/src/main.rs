extern crate aws_lambda as lambda;
extern crate aws_lambda_tower_web;
#[macro_use] extern crate tower_web;
#[macro_use] extern crate log;
extern crate rusoto_core;
extern crate rusoto_dynamodb;
extern crate serde;
extern crate serde_dynamodb;
extern crate chrono;
#[macro_use] extern crate maplit;

use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, PutItemInput, GetItemInput, DynamoDb, DynamoDbClient};

use tower_web::ServiceBuilder;
use aws_lambda_tower_web::ServiceBuilderExt;
use tower_web::error::DefaultCatch;
use tower_web::middleware::Identity;
use std::net::SocketAddr;
use chrono::DateTime;
use chrono::Utc;

struct Test {
    games: Games,
}

#[derive(Serialize, Deserialize, Debug, Response)]
struct Game {
    game_id: String,
    time_started: DateTime<Utc>,
}

struct Games {
    dynamodb: DynamoDbClient,
    table_name: String,
}

const GAME_ID: &str = "game_id";

impl Games {
    fn create_new_game(&self, game_id: &str) {
        info!("Accessing table with name: {}", self.table_name);
        let v = serde_dynamodb::to_hashmap(&Game {
            game_id: game_id.to_owned(),
            time_started: Utc::now(),
        }).unwrap();
        self.dynamodb.put_item(&PutItemInput {
            table_name: self.table_name.clone(),
            item: v,
            ..Default::default()
        }).sync().unwrap();
    }

    fn get_game(&self, game_id: &str) -> Game {
        let key_val = AttributeValue { s: Some(game_id.to_owned()), ..Default::default() };
        let thing = self.dynamodb.get_item(&GetItemInput {
            consistent_read: Some(true),
            key: hashmap! { GAME_ID.to_owned() => key_val },
            table_name: self.table_name.clone(),
            ..Default::default()
        }).sync().unwrap()
            .item.unwrap();

        let real_thing: Game = serde_dynamodb::from_hashmap(thing).unwrap();

        info!("Read: {:?}", real_thing);

        real_thing
    }
}

impl Test {
    fn create_game(&self, game_id: String) -> Result<String, ()> {
        self.games.create_new_game(&game_id);
        Ok(game_id)
    }

}

impl_web! {
    impl Test {
        #[get("/test")]
        fn test(&self) -> Result<&'static str, ()> {
            Ok("Hello ƛ!")
        }

        #[post("/create/:game_id")]
        fn create(&self, game_id: String) -> Result<String, ()> {
            self.create_game(game_id)
        }

        #[get("/games/:game_id")]
        #[content_type("json")]
        fn get_game(&self, game_id: String) -> Result<Game, ()> {
            Ok(self.games.get_game(&game_id))
        }
    }
}

fn service_builder(region: Region, games_table_name: String) -> ServiceBuilder<Test, DefaultCatch, Identity> {
    lambda::logger::init();
    info!("New lambda started!");

    let dynamodb = DynamoDbClient::simple(region);

    ServiceBuilder::new()
        .resource(Test { games: Games { dynamodb, table_name: games_table_name } })
}

fn main() {
    service_builder(Region::EuWest2, std::env::var("GAMES_TABLE").unwrap())
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
        }, "extreme-startup-prod-games".to_owned())
            .run(&addr)
            .unwrap();
    }
}