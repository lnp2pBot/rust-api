use rocket::*;
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

#[get("/community")]
pub fn get_communities() -> String {
    unimplemented!()
}
