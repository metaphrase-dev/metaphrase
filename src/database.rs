use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use errors::StringError;
use std::env;

pub fn establish_connection() -> Result<SqliteConnection, StringError> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let busy_timeout = env::var("DATABASE_BUSY_TIMEOUT").unwrap_or("250".to_string());

    let connection = try!(SqliteConnection::establish(&database_url));

    try!(connection.execute(format!("PRAGMA busy_timeout = {};", busy_timeout).as_str()));

    Ok(connection)
}
