extern crate url;
extern crate serde_json;

use std::fmt;
use std::error;
use std::io;
use url::ParseError;

#[derive(Debug)]
pub enum HttpError {
    Parse(ParseError),
    IO(io::Error),
    Json(serde_json::Error),
}

impl From<ParseError> for HttpError {
    fn from(err: ParseError) -> HttpError {
        HttpError::Parse(err)
    }
}

impl From<io::Error> for HttpError {
    fn from(err: io::Error) -> HttpError {
        HttpError::IO(err)
    }
}

impl From<serde_json::Error> for HttpError {
    fn from(err: serde_json::Error) -> HttpError {
        HttpError::Json(err)
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HttpError::Parse(ref err) => write!(f, "Parse error: {}", err),
            HttpError::IO(ref err) => write!(f, "Parse error: {}", err),
            HttpError::Json(ref err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl error::Error for HttpError {
    fn description(&self) -> &str {
        match *self {
            HttpError::Parse(ref err) => err.description(),
            HttpError::IO(ref err) => err.description(),
            HttpError::Json(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            HttpError::Parse(ref err) => Some(err),
            HttpError::IO(ref err) => Some(err),
            HttpError::Json(ref err) => Some(err),
        }
    }
}
