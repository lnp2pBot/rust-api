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
use routes::{
    bad_request, get_communities, get_community, get_order, get_orders, get_orders_stats, get_user,
    index, not_found, server_error,
};

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
        .register(
            "/",
            catchers!(
                not_found,
                bad_request,
                server_error,
                rocket_governor_catcher
            ),
        )
        .mount(
            "/",
            routes![
                index,
                get_user,
                get_order,
                get_orders,
                get_community,
                get_communities,
                get_orders_stats,
            ],
        )
        .attach(CORS)
}
