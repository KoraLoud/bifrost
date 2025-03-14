use std::error::Error;
use std::fmt;

pub type Result<T> = std::result::Result<T, HttpParseError>;

#[derive(Debug, Clone)]
pub enum HttpParseError {
    ParseError,
    OtherError(String),
}

impl Error for HttpParseError {}

impl From<http::method::InvalidMethod> for HttpParseError {
    fn from(error: http::method::InvalidMethod) -> Self {
        HttpParseError::OtherError(error.to_string())
    }
}

impl From<http::uri::InvalidUri> for HttpParseError {
    fn from(error: http::uri::InvalidUri) -> Self {
        HttpParseError::OtherError(error.to_string())
    }
}

impl From<http::Error> for HttpParseError {
    fn from(error: http::Error) -> Self {
        HttpParseError::OtherError(error.to_string())
    }
}

impl fmt::Display for HttpParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            Self::ParseError => "Error parsing http packet!",
            Self::OtherError(str) => str,
        };
        write!(formatter, "Error: {message}")
    }
}
