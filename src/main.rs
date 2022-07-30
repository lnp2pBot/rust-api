pub mod db;
pub mod routes;

#[macro_use]
extern crate rocket;
// External constructors
extern crate serde;
extern crate serde_json;

use rocket_governor::rocket_governor_catcher;

use routes::{index, get_communities, get_community};
use db::mongo::DBMongo;

#[launch]
async fn rocket() -> _ {
    let db = DBMongo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![index, get_communities, get_community])
        .register("/", catchers!(rocket_governor_catcher))
}
