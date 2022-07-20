use mongodb::{Client, options::ClientOptions};
use dotenv::dotenv;
use std::env;
use mongodb::Database;

pub async fn get_database() -> Database {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
    
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse(database_url).await.expect("Error getting client options");
    
    // Manually set an option.
    client_options.app_name = Some("lnp2pbot".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options).expect("Error getting db client");

    let db = client.database(&database_name);

    db
}
