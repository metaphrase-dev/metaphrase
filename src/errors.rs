use diesel::ConnectionError as DieselConnectionError;
use diesel::result::Error as DieselResultError;
use iron::prelude::*;
use iron::status;
use std::error::Error;
use std::fmt::{self, Debug};
use std::string::FromUtf8Error;
use time::ParseError;

#[derive(Debug)]
pub struct BadRequestError(pub String);

impl fmt::Display for BadRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for BadRequestError {
    fn description(&self) -> &str { &*self.0.as_str() }
}

impl From<BadRequestError> for IronError {
    fn from(error: BadRequestError) -> IronError {
        IronError::new(error, status::BadRequest)
    }
}

#[derive(Debug)]
pub struct NotFoundError(pub String);

impl fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for NotFoundError {
    fn description(&self) -> &str { &*self.0.as_str() }
}

impl From<NotFoundError> for IronError {
    fn from(error: NotFoundError) -> IronError {
        IronError::new(error, status::NotFound)
    }
}

// FIXME: Remove this generic error once all errors are implemented
#[derive(Debug)]
pub struct StringError(pub &'static str);

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for StringError {
    fn description(&self) -> &str { &*self.0 }
}

impl PartialEq for StringError {
    fn eq(&self, other: &StringError) -> bool {
        self.description() == other.description()
    }
}

impl From<DieselConnectionError> for StringError {
    fn from(_: DieselConnectionError) -> StringError {
        StringError("Database connection error")
    }
}

impl From<DieselResultError> for StringError {
    fn from(_: DieselResultError) -> StringError {
        StringError("ORM result error")
    }
}

impl From<FromUtf8Error> for StringError {
    fn from(_: FromUtf8Error) -> StringError {
        StringError("Error when parsing bytes to String")
    }
}

impl From<ParseError> for StringError {
    fn from(_: ParseError) -> StringError {
        StringError("Time parse error")
    }
}

impl From<StringError> for IronError {
    fn from(error: StringError) -> IronError {
        IronError::new(error, status::InternalServerError)
    }
}
