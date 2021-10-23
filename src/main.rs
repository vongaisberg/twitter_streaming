#![feature(cell_update)]

use futures::prelude::*;
use mongodb::bson::{to_document, Document};
use mongodb::{options::ClientOptions, Client};
use serde_json::Value;
use std::cell::Cell;

use std::time::Instant;

use twitter_stream::{Token, TwitterStream};

use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = Token::from_parts(
        env::var("CLIENT_IDENTIFIER").unwrap(),
        env::var("CLIENT_SECRET").unwrap(),
        env::var("TOKEN").unwrap(),
        env::var("TOKEN_SECRET").unwrap(),
    );
    //let mut url_map = HashMap::<String, u64>::new();

    let client = connect().await;
    let db = client.database("twitter");
    let tweets = db.collection::<Document>("tweets");
    //twitter_stream::Builder::new(token).
    let now = Cell::new(Instant::now());
    let count = Cell::new(0u64);
    let count_total = Cell::new(0u64);
    loop {
    TwitterStream::sample(&token)
        .try_flatten_stream()
        .for_each(|json| async {
            count.update(|v| v + 1);
            count_total.update(|v| v + 1);
            if now.get().elapsed().as_secs() > 10 {
                println!(
                    "Tweets pro Sekunde: {}, Gesamtanzahl: {}",
                    count.get() as f64 / now.get().elapsed().as_millis() as f64 * 1000f64,
                    count_total.get()
                );
                now.set(Instant::now());
                count.set(0);
            }
            match json {
                Ok(json) => {
                    let v: Value = serde_json::from_str(&json).unwrap();
                    tweets
                        .insert_one(to_document(&v).unwrap(), None)
                        .await
                        .unwrap();
                }

                Err(err) => {
                    println!("Could not unwrap json: {}", err);
                }
            }
        })
        .await;}
}

async fn connect() -> mongodb::Client {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://max.hadiko.de:27017")
        .await
        .unwrap();

    // Manually set an option.
    client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options).unwrap();

    println!("Created Client");

    // List the names of the databases in that deployment.
    for db_name in client.list_database_names(None, None).await.unwrap() {
        println!("{}", db_name);
    }
    println!("Connected");
    client
}
