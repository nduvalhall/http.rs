use std::{fs::File, io::Read};

use crate::{ContentType, FromBytes, HttpError, IntoBytes};

pub struct Html(pub String);

impl Html {
    pub fn new(html: &str) -> Self {
        Self(html.into())
    }

    pub fn from_file(filepath: &str) -> Result<Self, HttpError> {
        let mut html = String::new();
        let Ok(mut file) = File::open(filepath) else {
            return Err(HttpError::new(500, &format!("File {} not found", filepath)));
        };

        let Ok(_) = file.read_to_string(&mut html) else {
            return Err(HttpError::new(500, "Failed to read file"));
        };

        Ok(Html(html))
    }
}

impl FromBytes for Html {
    fn from_bytes(_: Vec<u8>) -> Result<Self, HttpError> {
        todo!()
    }
}

impl IntoBytes for Html {
    fn into_bytes(self) -> Vec<u8> {
        self.0.into_bytes()
    }
}

impl ContentType for Html {
    fn content_type(&self) -> &'static str {
        "text/html; charset=utf-8"
    }
}
