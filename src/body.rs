use crate::ContentType;

#[derive(Debug)]
pub struct Body {
    pub content_type: ContentType,
    pub data: Vec<u8>,
}

impl Body {
    pub fn new(content_type: ContentType, data: Vec<u8>) -> Self {
        Body { content_type, data }
    }
}
