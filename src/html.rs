use crate::{ContentType, FromBytes, HttpError, IntoBytes};

pub struct Html(pub String);

impl Html {
    pub fn new(html: &str) -> Self {
        Self(html.into())
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
