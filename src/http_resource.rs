use std::fs;
use std::fs::Metadata;
use std::io::Read;
use std::path::Path;
pub struct HttpResource {
    pub mime_type: String,
    pub file_ext: String,
    pub metadata: Metadata,
    pub file_data: Vec<u8>,
}

impl HttpResource {
    pub fn new(path: &str, extension: &str, mime_type: &str) -> HttpResource {
        let mut file = fs::File::open(Path::new(&path)).expect("Failed to read file {path}");
        let mut filedata: Vec<u8> = Vec::new();
        file.read_to_end(&mut filedata).unwrap();
        let file_metadata = file.metadata().unwrap();
        HttpResource {
            mime_type: String::from(mime_type),
            file_ext: String::from(extension),
            metadata: file_metadata,
            file_data: filedata,
        }
    }

    pub fn is_utf8(&self) -> bool {
        let utf_8_string = String::from_utf8(self.file_data.clone());
        utf_8_string.is_ok()
    }
}
