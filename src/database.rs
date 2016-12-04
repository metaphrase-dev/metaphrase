use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use errors::StringError;
use std::env;

pub fn establish_connection() -> Result<SqliteConnection, StringError> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let connection = try!(SqliteConnection::establish(&database_url));

    Ok(connection)
}
