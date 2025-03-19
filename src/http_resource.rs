use std::fs::Metadata;
use std::io::Read;
use std::path::Path;
use std::{fmt, fs};
pub struct HttpResource {
    pub file_ext: String,
    pub metadata: Metadata,
    pub file_data: String,
}

impl HttpResource {
    pub fn new(path: &str, extension: &str) -> HttpResource {
        let mut file = fs::File::open(Path::new(&path)).expect("Failed to read file {path}");
        let mut filedata = String::new();
        file.read_to_string(&mut filedata).unwrap();
        let file_metadata = file.metadata().unwrap();
        HttpResource {
            file_ext: String::from(extension),
            metadata: file_metadata,
            file_data: filedata,
        }
    }
}
