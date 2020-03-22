use hyper;
use serde_json;
use std::{cell, error::Error, fmt, io, string};

// Errors
#[derive(Debug)]
pub enum TwilioErr {
    Io(io::Error),
    UrlParse(http::uri::InvalidUri),
    NetworkErr(hyper::Error),
    SerdeErr(serde_json::Error),
    BorrowErr(cell::BorrowMutError),
    Utf8Err(string::FromUtf8Error),
    HttpErr(http::Error),
    HeaderErr(typed_headers::Error),
}

pub use super::TwilioErr::*;

pub type TwilioResult<T> = Result<T, TwilioErr>;

impl Error for TwilioErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            Io(ref e) => e.source(),
            SerdeErr(ref e) => e.source(),
            UrlParse(ref e) => e.source(),
            NetworkErr(ref e) => e.source(),
            BorrowErr(ref e) => e.source(),
            Utf8Err(ref e) => e.source(),
            HttpErr(ref e) => e.source(),
            HeaderErr(ref e) => e.source(),
        }
    }
}

impl fmt::Display for TwilioErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Io(ref e) => write!(f, "IO Error: {}", e),
            SerdeErr(ref e) => write!(f, "Serde JSON Error: {}", e),
            UrlParse(ref e) => write!(f, "URL parse error: {}", e),
            NetworkErr(ref e) => write!(f, "There was a network error. {}", e),
            BorrowErr(ref e) => write!(f, "Error trying to get client reference. {}", e),
            Utf8Err(ref e) => write!(f, "Error converting to utf-8 string: {}", e),
            HttpErr(ref e) => write!(f, "Http error when building req: {}", e),
            HeaderErr(ref e) => write!(f, "Error creating header value: {}", e),
        }
    }
}

from!(cell::BorrowMutError, BorrowErr);
from!(hyper::Error, NetworkErr);
from!(serde_json::Error, SerdeErr);
from!(http::uri::InvalidUri, UrlParse);
from!(io::Error, Io);
from!(string::FromUtf8Error, Utf8Err);
from!(http::Error, HttpErr);
from!(typed_headers::Error, HeaderErr);
