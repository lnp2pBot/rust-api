use dotenv::dotenv;
use mongodb::sync::{Client, Database};
use std::env;

pub fn get_database() -> Database {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");

    let client = Client::with_uri_str(database_url).unwrap();

    client.database(&database_name)
}
