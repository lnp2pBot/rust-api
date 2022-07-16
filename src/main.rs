#[macro_use] extern crate rocket;

pub mod routes;
pub mod db;

use routes as api_routes;

#[launch]
async fn rocket() -> _ {
    db::connect().await;
    rocket::build().mount("/", routes![api_routes::index])
}