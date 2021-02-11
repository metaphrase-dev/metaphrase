use super::errors::LughError;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::env;

pub fn establish_connection() -> Result<SqliteConnection, LughError> {
    let database_url = env::var("DATABASE_URL").unwrap();
    let busy_timeout = env::var("DATABASE_BUSY_TIMEOUT").unwrap_or_else(|_| "250".to_string());

    let connection = SqliteConnection::establish(&database_url)?;

    connection.execute(format!("PRAGMA busy_timeout = {};", busy_timeout).as_str())?;

    Ok(connection)
}
