use crate::db::mongo::DBMongo;
use crate::db::schemas::{Community, CommunityRequest, Order, OrderRequest, User};
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

#[get("/communities?<_id>&<currency>")]
pub fn get_communities(
    db: &State<DBMongo>,
    _id: Option<String>,
    currency: Option<String>,
) -> Result<Json<Vec<Community>>, Status> {
    let params = CommunityRequest { _id, currency };
    let comms = db.get_communities(&params);

    match comms {
        Ok(o) => Ok(Json(o)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/user/<id>")]
pub fn get_user(db: &State<DBMongo>, id: &str) -> Result<Json<User>, Status> {
    let comm = db.get_user(id);

    match comm {
        Ok(o) => Ok(Json(o)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/community/<id>")]
pub fn get_community(db: &State<DBMongo>, id: &str) -> Result<Json<Community>, Status> {
    let comm = db.get_community(id);

    match comm {
        Ok(o) => Ok(Json(o)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/order/<id>")]
pub fn get_order(db: &State<DBMongo>, id: &str) -> Result<Json<Order>, Status> {
    let order = db.get_order(id);

    match order {
        Ok(o) => Ok(Json(o)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/orders?<_id>&<direction>&<currency>&<community_id>")]
pub fn get_orders(
    db: &State<DBMongo>,
    _id: Option<String>,
    direction: Option<String>,
    currency: Option<String>,
    community_id: Option<String>,
) -> Result<Json<Vec<Order>>, Status> {
    let params = OrderRequest {
        _id,
        status: None,
        direction,
        currency,
        community_id,
    };
    let orders = db.get_orders(&params);

    match orders {
        Ok(o) => Ok(Json(o)),
        Err(_) => Err(Status::InternalServerError),
    }
}
