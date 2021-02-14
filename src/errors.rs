use actix_web::ResponseError;
use diesel::result::Error as DieselResultError;
use diesel::ConnectionError as DieselConnectionError;
use std::error::Error;
use std::fmt;
use std::string::FromUtf8Error;
use time::ParseError;

#[derive(Debug)]
pub enum MetaphraseError {
    BadRequest(String),
    DatabaseError(String),
    NotFound(String),
    ParseFailed(String),
    Unauthorized(String),
}

impl fmt::Display for MetaphraseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MetaphraseError::BadRequest(ref message)
            | MetaphraseError::DatabaseError(ref message)
            | MetaphraseError::NotFound(ref message)
            | MetaphraseError::ParseFailed(ref message)
            | MetaphraseError::Unauthorized(ref message) => f.write_str(message),
        }
    }
}

impl Error for MetaphraseError {
    fn description(&self) -> &str {
        match *self {
            MetaphraseError::BadRequest(ref message)
            | MetaphraseError::DatabaseError(ref message)
            | MetaphraseError::NotFound(ref message)
            | MetaphraseError::ParseFailed(ref message)
            | MetaphraseError::Unauthorized(ref message) => message.as_str(),
        }
    }
}

impl ResponseError for MetaphraseError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            MetaphraseError::BadRequest(_) => actix_web::http::StatusCode::BAD_REQUEST,
            MetaphraseError::DatabaseError(_) | MetaphraseError::ParseFailed(_) => {
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            }
            MetaphraseError::NotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
            MetaphraseError::Unauthorized(_) => actix_web::http::StatusCode::UNAUTHORIZED,
        }
    }
}

impl PartialEq for MetaphraseError {
    fn eq(&self, other: &MetaphraseError) -> bool {
        self.to_string() == other.to_string()
    }
}

impl From<DieselConnectionError> for MetaphraseError {
    fn from(_: DieselConnectionError) -> MetaphraseError {
        MetaphraseError::DatabaseError("Diesel connection error".to_string())
    }
}

impl From<DieselResultError> for MetaphraseError {
    fn from(_: DieselResultError) -> MetaphraseError {
        MetaphraseError::DatabaseError("Diesel result error".to_string())
    }
}

impl From<FromUtf8Error> for MetaphraseError {
    fn from(_: FromUtf8Error) -> MetaphraseError {
        MetaphraseError::ParseFailed("Parse from UTF8 error".to_string())
    }
}

impl From<ParseError> for MetaphraseError {
    fn from(_: ParseError) -> MetaphraseError {
        MetaphraseError::ParseFailed("Parse error".to_string())
    }
}
