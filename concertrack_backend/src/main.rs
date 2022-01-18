use application::tracked;

#[macro_use]
extern crate rocket;
extern crate dotenv;
#[macro_use]
extern crate diesel;

mod application;
mod data_access;

#[get("/")]
fn index() -> &'static str {
    tracked::get_tracked(String::new());
    "Hello, world!"
}

#[get("/tracked/<id>")]
fn get_tracked(id: String) -> String {
    id
}

#[post("/tracked/<id>", data = "<artist>")]
fn add_tracked(id: String, artist: String) -> String {
    id + " += " + &artist
}

#[delete("/tracked/<id>", data = "<artist>")]
fn remove_tracked(id: String, artist: String) -> String {
    id + " -= " + &artist
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_tracked, add_tracked, remove_tracked])
}
