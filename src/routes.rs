use crate::db::mongo::DBMongo;
use crate::db::schemas::{
    Community, CommunityRequest, Order, OrderRequest, OrdersStatsRequest, User,
};
use mongodb::bson::Document;
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
    let user = db.get_user(id);
    if user.is_err() {
        return Err(Status::BadRequest);
    }

    match user {
        Ok(Some(u)) => Ok(Json(u)),
        _ => Err(Status::NotFound),
    }
}

#[get("/community/<id>")]
pub fn get_community(db: &State<DBMongo>, id: &str) -> Result<Json<Community>, Status> {
    let comm = db.get_community(id);
    if comm.is_err() {
        return Err(Status::BadRequest);
    }

    match comm {
        Ok(Some(c)) => Ok(Json(c)),
        _ => Err(Status::NotFound),
    }
}

#[get("/order/<id>")]
pub fn get_order(db: &State<DBMongo>, id: &str) -> Result<Json<Order>, Status> {
    let order = db.get_order(id);
    if order.is_err() {
        return Err(Status::BadRequest);
    }

    match order {
        Ok(Some(o)) => Ok(Json(o)),
        _ => Err(Status::NotFound),
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

#[get("/orders_stats?<direction>&<community_id>&<date_from>&<date_to>&<status>")]
pub fn get_orders_stats(
    db: &State<DBMongo>,
    direction: Option<String>,
    community_id: Option<String>,
    date_from: Option<String>,
    date_to: Option<String>,
    status: Option<String>,
) -> Result<Json<Vec<Document>>, Status> {
    let params = OrdersStatsRequest {
        date_from,
        date_to,
        status,
        direction,
        community_id,
    };
    let stats = db.get_orders_stats(&params);

    match stats {
        Ok(o) => Ok(Json(o)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[catch(400)]
pub fn bad_request() -> String {
    "Bad request".to_string()
}

#[catch(404)]
pub fn not_found() -> String {
    "We couldn't find what you are looking for".to_string()
}

#[catch(500)]
pub fn server_error() -> String {
    "Internal server error :(".to_string()
}
