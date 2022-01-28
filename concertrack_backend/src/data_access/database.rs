use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use crate::rocket::{debug, error};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    match PgConnection::establish(&database_url) {
        Ok(connection) => {
            debug!("Established connection to {}", database_url);
            return connection;
        }
        Err(error) => {
            error!("Error connecting to {}", database_url);
            panic!("{:?}", error);
        }
    }
}
