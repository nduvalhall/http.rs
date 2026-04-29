use std::collections::HashMap;

use crate::http_error::HttpError;

pub struct RawRequest {
    pub version: String,
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

pub trait FromRawRequest: Sized {
    fn from_raw_request(raw_request: RawRequest) -> Result<Self, HttpError>;
}
