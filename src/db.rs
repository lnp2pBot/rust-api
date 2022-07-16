use mongodb::{Client, options::ClientOptions};
use dotenv::dotenv;
use std::env;

pub async fn connect() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse(database_url).await.expect("Error getting client options");

    // Manually set an option.
    client_options.app_name = Some("lnp2pbot".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options).expect("Error getting db client");

    // List the names of the databases in that deployment.
    for db_name in client.list_database_names(None, None).await.expect("Error getting db names") {
        println!("{}", db_name);
    }
}
