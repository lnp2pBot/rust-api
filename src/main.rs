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
    let comm_coll = db.collection::<db::models::Community>("communities");
    let comm = comm_coll
        .find_one(doc! {"name": "Tropykus P2P"}, None)
        .await
        .unwrap()
        .unwrap();
    println!("{comm:?}");
    let user_coll = db.collection::<db::models::User>("users");
    let user = user_coll
        .find_one(doc! {"username": "negrunch"}, None)
        .await
        .unwrap()
        .unwrap();
    println!("{user:?}");
    let dispute_coll = db.collection::<db::models::Dispute>("disputes");
    let dispute = dispute_coll
        .find_one(doc! {"status": "IN_PROGRESS"}, None)
        .await
        .unwrap()
        .unwrap();
    println!("{dispute:?}");
    let pp_coll = db.collection::<db::models::PendingPayment>("pendingpayments");
    let pp = pp_coll
        .find_one(doc! {"description": "Retiro por admin @negrunch"}, None)
        .await
        .unwrap()
        .unwrap();
    println!("{pp:?}");
    let order_coll = db.collection::<db::models::Order>("orders");
    let order = order_coll
        .find_one(doc! {"fiat_code": "ARS"}, None)
        .await
        .unwrap()
        .unwrap();
    println!("{order:?}");
    rocket::build().mount("/", routes![api_routes::index])
}
