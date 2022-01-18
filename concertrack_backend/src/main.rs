#[macro_use]
extern crate rocket;
extern crate dotenv;
#[macro_use]
extern crate diesel;

// declare modules
mod application;
mod data_access;

use application::tracked;

use rocket::serde::json::Json;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/tracked/<code>")]
fn get_tracked(code: String) -> Json<Vec<String>> {
    Json(tracked::get_tracked(code))
}

#[post("/tracked/<code>", data = "<artist>")]
fn add_tracked(code: String, artist: String) {
    tracked::add_tracked(code, artist)
}

#[delete("/tracked/<code>", data = "<artist>")]
fn remove_tracked(code: String, artist: String) {
    tracked::delete_tracked(code, artist)
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_tracked, add_tracked, remove_tracked])
}
