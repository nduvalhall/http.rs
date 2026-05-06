use crate::response::ContentType;

/// A request or response body: raw bytes with an associated content type.
#[derive(Debug)]
pub struct Body {
    /// MIME type of the body content.
    pub content_type: ContentType,
    /// Raw body bytes.
    pub data: Vec<u8>,
}

impl Body {
    /// Creates a new body from a content type and raw bytes.
    pub fn new(content_type: ContentType, data: Vec<u8>) -> Self {
        Body { content_type, data }
    }
}
