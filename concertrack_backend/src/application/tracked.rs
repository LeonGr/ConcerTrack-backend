// extern crate diesel;

// use crate::diesel::prelude::*;
use crate::data_access::database;

pub fn get_tracked(track_code: String) {
    // use crate::data_access::schema::tracked::dsl::*;

    let _connection = database::establish_connection();

    println!("Track code: {}", track_code);
}
