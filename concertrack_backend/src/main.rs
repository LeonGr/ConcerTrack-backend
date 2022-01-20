#[macro_use]
extern crate rocket;
extern crate dotenv;
#[macro_use]
extern crate diesel;

// declare modules
mod application;
mod data_access;

use application::tracked;

use rocket::{serde::json::Json, http::Method};
use rocket_cors::{AllowedOrigins, AllowedHeaders};

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
    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::some_exact(&["https://concertrack.com", "http://localhost:8080"]),
        allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Accept", "Content-Type"]),
        ..Default::default()
    }
    .to_cors()
    .expect("Could not create CORS options");

    rocket::build()
        .mount("/", routes![index, get_tracked, add_tracked, remove_tracked])
        .mount("/", rocket_cors::catch_all_options_routes())
        .attach(cors.clone())
        .manage(cors)
}
