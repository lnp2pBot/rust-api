use crate::db::mongo::DBMongo;
use crate::db::schemas::{Community, Order, OrderRequest};
use rocket::*;
use rocket::{http::Status, serde::json::Json, State};
use rocket_governor::{Method, Quota, RocketGovernable, RocketGovernor};

pub struct RateLimitGuard;

impl<'r> RocketGovernable<'r> for RateLimitGuard {
    fn quota(_method: Method, _route_name: &str) -> Quota {
        Quota::per_minute(Self::nonzero(10u32))
    }
}

#[get("/")]
pub fn index(_limitguard: RocketGovernor<RateLimitGuard>) -> &'static str {
    "Hello, world!"
}

#[get("/communities")]
pub fn get_communities(db: &State<DBMongo>) -> Result<Json<Vec<Community>>, Status> {
    let comms = db.get_communities();

    match comms {
        Ok(o) => Ok(Json(o)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/community/<id>")]
pub fn get_community(db: &State<DBMongo>, id: &str) -> Result<Json<Community>, Status> {
    let comm = db.get_community(&id);

    match comm {
        Ok(o) => Ok(Json(o)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/order/<id>")]
pub fn get_order(db: &State<DBMongo>, id: &str) -> Result<Json<Order>, Status> {
    let comm = db.get_order(&id);

    match comm {
        Ok(o) => Ok(Json(o)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/orders", format = "json", data = "<params>")]
pub fn get_orders(
    db: &State<DBMongo>,
    params: Json<OrderRequest>,
) -> Result<Json<Vec<Order>>, Status> {
    let comms = db.get_orders(&params);

    match comms {
        Ok(o) => Ok(Json(o)),
        Err(_) => Err(Status::InternalServerError),
    }
}
