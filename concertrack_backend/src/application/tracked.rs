use crate::diesel::prelude::*;
use crate::data_access::{database, model::{ TrackEntry, NewEntry }};

use crate::rocket::{error, info};

pub fn get_tracked(track_code: String) -> Vec<String> {
    use crate::data_access::schema::tracked::dsl::*;

    let connection = database::establish_connection();

    let entry_result = tracked
        .filter(code.eq(track_code.clone()))
        .load::<TrackEntry>(&connection);

    match entry_result {
        Ok(track_entries) => {
            info!("Retrieved results for code {}: {:?}", track_code, track_entries.len());

            track_entries
                .iter()
                .map(|entry| entry.artist.clone())
                .collect()
        },
        Err(error) => {
            error!("Error loading tracked artist entries");
            panic!("{:?}", error);
        }
    }
}

pub fn add_tracked(track_code: String, artist_name: String) {
    use crate::data_access::schema::tracked;
    use crate::data_access::schema::tracked::dsl::*;

    let connection = database::establish_connection();

    let existing_entry_result = tracked
        .filter(code.eq(track_code.clone()).and(artist.eq(artist_name.clone())))
        .load::<TrackEntry>(&connection);

    match existing_entry_result {
        Ok(existing_track_entries) => {
            if existing_track_entries.len() > 0 {
                info!("Artist ({artist_name}) already tracked for code {track_code}");
                return
            }

            let new_entry = NewEntry {
                code: track_code.clone(),
                artist: artist_name.clone(),
            };

            let execution_result =
                diesel::insert_into(tracked::table)
                    .values(&new_entry)
                    .execute(&connection);

            match execution_result {
                Ok(affected) => {
                    info!("Added {affected} artist ({artist_name}) for code {track_code}");

                    assert_eq!(1, affected)
                }
                Err(error) => {
                    error!("Error adding new tracked artist ({artist_name}) entry for code {track_code}");
                    panic!("{:?}", error);
                }
            }

        }
        Err(error) => {
            error!("Error loading tracked artist entries");
            panic!("{:?}", error);
        }
    }
}


pub fn delete_tracked(track_code: String, artist_name: String) {
    use crate::data_access::schema::tracked::dsl::*;

    let connection = database::establish_connection();

    let execution_result =
        diesel::delete(tracked.filter(code.eq(track_code.clone()).and(artist.eq(artist_name.clone()))))
            .execute(&connection);

    match execution_result {
        Ok(affected) => {
            if affected == 0 {
                info!("No artist ({artist_name}) tracked for code {track_code}");
            } else {
                info!("Removed {affected} artist ({artist_name}) for code {track_code}");
            }

            assert!(affected < 2)
        }
        Err(error) => {
            error!("Error adding new tracked artist entry");
            panic!("{:?}", error);
        }
    }
}
