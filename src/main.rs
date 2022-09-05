pub mod db;
pub mod routes;

#[macro_use]
extern crate rocket;
// External constructors
extern crate serde;
extern crate serde_json;

use rocket_governor::rocket_governor_catcher;

use db::mongo::DBMongo;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};
use routes::{get_communities, get_community, get_order, get_orders, get_user, index};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
async fn rocket() -> _ {
    let db = DBMongo::init();
    rocket::build()
        .manage(db)
        .mount(
            "/",
            routes![
                index,
                get_communities,
                get_community,
                get_orders,
                get_order,
                get_user
            ],
        )
        .register("/", catchers!(rocket_governor_catcher))
        .attach(CORS)
}
