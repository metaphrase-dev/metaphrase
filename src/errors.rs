use diesel::ConnectionError as DieselConnectionError;
use diesel::result::Error as DieselResultError;
use iron::prelude::*;
use iron::status;
use std::error::Error;
use std::fmt;
use std::string::FromUtf8Error;
use params::ParamsError;
use time::ParseError;

#[derive(Debug)]
pub enum LughError {
    BadRequest(String),
    DatabaseError(String),
    NotFound(String),
    ParseFailed(String),
    Unauthorized(String)
}

impl fmt::Display for LughError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LughError::BadRequest(ref message) |
                LughError::DatabaseError(ref message) |
                LughError::NotFound(ref message) |
                LughError::ParseFailed(ref message) |
                LughError::Unauthorized(ref message)
            => f.write_str(message),
        }
    }
}

impl Error for LughError {
    fn description(&self) -> &str {
        match *self {
            LughError::BadRequest(ref message) |
                LughError::DatabaseError(ref message) |
                LughError::NotFound(ref message) |
                LughError::ParseFailed(ref message) |
                LughError::Unauthorized(ref message)
            => message.as_str(),
        }
    }
}

impl From<LughError> for IronError {
    fn from(error: LughError) -> IronError {
        match error {
            LughError::BadRequest(_) => IronError::new(error, status::BadRequest),
            LughError::DatabaseError(_) | LughError::ParseFailed(_) => IronError::new(error, status::InternalServerError),
            LughError::NotFound(_) => IronError::new(error, status::NotFound),
            LughError::Unauthorized(_) => IronError::new(error, status::Unauthorized),
        }
    }
}

impl PartialEq for LughError {
    fn eq(&self, other: &LughError) -> bool {
        self.description() == other.description()
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

impl From<ParamsError> for LughError {
    fn from(error: ParamsError) -> LughError {
        LughError::BadRequest(error.description().to_string())
    }
}

impl From<ParseError> for LughError {
    fn from(_: ParseError) -> LughError {
        LughError::ParseFailed("Parse error".to_string())
    }
}

