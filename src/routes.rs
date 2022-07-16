use rocket::*;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/community")]
pub fn get_communities() -> String {
    unimplemented!()
}