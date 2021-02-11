use actix_web::ResponseError;
use diesel::result::Error as DieselResultError;
use diesel::ConnectionError as DieselConnectionError;
use std::error::Error;
use std::fmt;
use std::string::FromUtf8Error;
use time::ParseError;

#[derive(Debug)]
pub enum LughError {
    BadRequest(String),
    DatabaseError(String),
    NotFound(String),
    ParseFailed(String),
    Unauthorized(String),
}

impl fmt::Display for LughError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LughError::BadRequest(ref message)
            | LughError::DatabaseError(ref message)
            | LughError::NotFound(ref message)
            | LughError::ParseFailed(ref message)
            | LughError::Unauthorized(ref message) => f.write_str(message),
        }
    }
}

impl Error for LughError {
    fn description(&self) -> &str {
        match *self {
            LughError::BadRequest(ref message)
            | LughError::DatabaseError(ref message)
            | LughError::NotFound(ref message)
            | LughError::ParseFailed(ref message)
            | LughError::Unauthorized(ref message) => message.as_str(),
        }
    }
}

impl ResponseError for LughError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            LughError::BadRequest(_) => actix_web::http::StatusCode::BAD_REQUEST,
            LughError::DatabaseError(_) | LughError::ParseFailed(_) => {
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            }
            LughError::NotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
            LughError::Unauthorized(_) => actix_web::http::StatusCode::UNAUTHORIZED,
        }
    }
}

impl PartialEq for LughError {
    fn eq(&self, other: &LughError) -> bool {
        self.to_string() == other.to_string()
    }
}

impl From<DieselConnectionError> for LughError {
    fn from(_: DieselConnectionError) -> LughError {
        LughError::DatabaseError("Diesel connection error".to_string())
    }
}

impl From<DieselResultError> for LughError {
    fn from(_: DieselResultError) -> LughError {
        LughError::DatabaseError("Diesel result error".to_string())
    }
}

impl From<FromUtf8Error> for LughError {
    fn from(_: FromUtf8Error) -> LughError {
        LughError::ParseFailed("Parse from UTF8 error".to_string())
    }
}

impl From<ParseError> for LughError {
    fn from(_: ParseError) -> LughError {
        LughError::ParseFailed("Parse error".to_string())
    }
}
