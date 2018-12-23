use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;

use std::env;
use std::process;

pub fn db_connect() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
