extern crate serde_json;
extern crate native_tls;

use std::fmt;
use std::error;
use std::io;
use std::net::TcpStream;
use std::num::ParseIntError;
use url::ParseError;
use native_tls::HandshakeError;

#[derive(Debug)]
pub enum HttpError {
    Parse(ParseError),
    IO(io::Error),
    Json(serde_json::Error),
    TLS(native_tls::Error),
    SSL(HandshakeError<TcpStream>),
    ParseInt(ParseIntError),
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

impl From<native_tls::Error> for HttpError {
    fn from(err: native_tls::Error) -> HttpError {
        HttpError::TLS(err)
    }
}

impl From<HandshakeError<TcpStream>> for HttpError {
    fn from(err: HandshakeError<TcpStream>) -> HttpError {
        HttpError::SSL(err)
    }
}

impl From<ParseIntError> for HttpError {
    fn from(err: ParseIntError) -> HttpError {
        HttpError::ParseInt(err)
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HttpError::Parse(ref err) => write!(f, "Parse error: {}", err),
            HttpError::IO(ref err) => write!(f, "Parse error: {}", err),
            HttpError::Json(ref err) => write!(f, "Parse error: {}", err),
            HttpError::TLS(ref err) => write!(f, "Parse error: {}", err),
            HttpError::SSL(ref err) => write!(f, "Parse error: {}", err),
            HttpError::ParseInt(ref err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl error::Error for HttpError {
    fn description(&self) -> &str {
        match *self {
            HttpError::Parse(ref err) => err.description(),
            HttpError::IO(ref err) => err.description(),
            HttpError::Json(ref err) => err.description(),
            HttpError::TLS(ref err) => err.description(),
            HttpError::SSL(ref err) => err.description(),
            HttpError::ParseInt(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            HttpError::Parse(ref err) => Some(err),
            HttpError::IO(ref err) => Some(err),
            HttpError::Json(ref err) => Some(err),
            HttpError::TLS(ref err) => Some(err),
            HttpError::SSL(ref err) => Some(err),
            HttpError::ParseInt(ref err) => Some(err),
        }
    }
}
