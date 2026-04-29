use std::collections::HashMap;

use crate::{
    http_error::HttpError,
    raw_request::{FromRawRequest, RawRequest},
};

pub struct Request<T> {
    pub version: String,
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Option<T>,
}

pub trait FromBytes: Sized {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self, HttpError>;
}

impl<T: FromBytes> FromRawRequest for Request<T> {
    fn from_raw_request(raw_request: RawRequest) -> Result<Self, HttpError> {
        let body = match raw_request.body {
            Some(body) => Some(T::from_bytes(body)?),
            None => None,
        };

        Ok(Request {
            version: raw_request.version,
            method: raw_request.method,
            path: raw_request.path,
            headers: raw_request.headers,
            body: body,
        })
    }
}
