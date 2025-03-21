use std::{fmt, io};
pub mod http_parse_error;
pub mod http_resource;
pub mod http_response;
//pub mod mimetype_table;
pub mod thread_pool;

#[derive(Debug, Clone)]
pub struct DirectoryReadError {
    pub msg: String,
}

impl From<io::Error> for DirectoryReadError {
    fn from(error: io::Error) -> DirectoryReadError {
        DirectoryReadError {
            msg: error.to_string(),
        }
    }
}

impl fmt::Display for DirectoryReadError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Bifrost failed to read directory: {}", self.msg)
    }
}
