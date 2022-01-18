use crate::diesel::prelude::*;
use crate::data_access::{database, model::{ TrackEntry, NewEntry }};

pub fn get_tracked(track_code: String) -> Vec<String> {
    use crate::data_access::schema::tracked::dsl::*;

    let connection = database::establish_connection();

    let results = tracked
        .filter(code.eq(track_code.clone()))
        .load::<TrackEntry>(&connection)
        .expect("Error loading tracked artist entries");

    println!("Retrieved results for code {}: {:?}", track_code, results);

    results
        .iter()
        .map(|entry| entry.artist.clone())
        .collect()
}

pub fn add_tracked(track_code: String, artist_name: String) {
    use crate::data_access::schema::tracked;
    use crate::data_access::schema::tracked::dsl::*;

    let connection = database::establish_connection();

    let existing = tracked
        .filter(code.eq(track_code.clone()).and(artist.eq(artist_name.clone())))
        .load::<TrackEntry>(&connection)
        .expect("Error loading tracked artist entries");

    if existing.len() > 0 {
        return
    }

    let new_entry = NewEntry {
        code: track_code,
        artist: artist_name,
    };

    let affected =
        diesel::insert_into(tracked::table)
            .values(&new_entry)
            .execute(&connection)
            .expect("Error adding new tracked artist entry");

    assert_eq!(1, affected)
}


pub fn delete_tracked(track_code: String, artist_name: String) {
    use crate::data_access::schema::tracked::dsl::*;

    let connection = database::establish_connection();

    let affected =
        diesel::delete(tracked.filter(code.eq(track_code).and(artist.eq(artist_name))))
            .execute(&connection)
            .expect("Error adding new tracked artist entry");

    assert!(affected < 2)
}
