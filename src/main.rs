#[macro_use]
extern crate rocket;
// External constructors
extern crate serde;
extern crate serde_json;

pub mod db;
pub mod routes;

use futures::TryFutureExt;
use mongodb::{bson::doc, options::FindOptions};
use mongodb::{error::Error, Collection};

use routes as api_routes;

#[launch]
async fn rocket() -> _ {
    let x = chrono::Duration::weeks(1);
    println!("{x:?}");
    let db = db::connect::get_database().await;
    let coll = db.collection::<db::models::Community>("communities");
    let user = coll
        .find_one(doc! {"name": "Tropykus P2P"}, None)
        .await
        .unwrap()
        .unwrap();
    println!("{user:?}");
    rocket::build().mount("/", routes![api_routes::index])
}
