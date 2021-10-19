use futures::prelude::*;
use mongodb::bson::{doc, to_document, Bson, Document};
use mongodb::{options::ClientOptions, Client};
use serde_json::{Result, Value};
use std::collections::HashMap;
use twitter_stream::builder::FilterLevel;
use twitter_stream::{Token, TwitterStream};
use url::Url;

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
    let mut url_map = HashMap::<String, u64>::new();

    let client = connect().await;
    let db = client.database("twitter");
    let tweets = db.collection::<Document>("tweets");
    //twitter_stream::Builder::new(token).

    TwitterStream::sample(&token)
        .try_flatten_stream()
        .for_each(|json| async {
            //println!("New");
            let v: Value = serde_json::from_str(&json.unwrap()).unwrap();
            println!("{}", v["text"]);
            // println!("{}\n{}\n", v["text"], v["entities"]["urls"]);try_
            /* if let Some(urls) = v["entities"]["urls"].as_array() {
                            if urls.len() > 0 {
                                println!("{}", urls[0]["expanded_url"].as_str().unwrap());

                            let urls2: Vec<Document> = urls
                                .iter()
                                .map(|u| {
                                    println!("{}", u);
                                    /*let bytes = u.to_string().as_bytes().to_owned();
                                    let slice = bytes.as_slice();
                                    println!("Slice: {:?}", slice);*/
                                    //Document::from_reader(slice).unwrap()
                                    to_document(u).unwrap()
                                })
                                .collect();
                                println!("Vec<Document>: {:?}", urls2);
            */
            tweets
                .insert_one(to_document(&v).unwrap(), None)
                .await
                .unwrap();

            /*
            for url_str in urls {
                if let Ok(url) = Url::parse(url_str["expanded_url"].as_str().unwrap()) {
                    let tld = url.host_str().unwrap();
                    *url_map.entry(tld.to_string()).or_insert(0) += 1;
                }
            }

            let mut count_vec: Vec<(&String, &u64)> = url_map.iter().collect();
            count_vec.sort_by(|a, b| b.1.cmp(a.1));
            let mut i = 0;
            for entry in count_vec {
                i = i + 1;
                if i > 20 {
                    break;
                }
                println!("{}: {}", entry.0, entry.1);
            }
            println!("\n\n");
            */

            //  future::ok(())
        })
        .await;
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
