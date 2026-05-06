use crate::{http_error::HttpError, request::Request, response::Response};

pub struct Route<C> {
    pub method: String,
    pub path: String,
    pub handler: fn(&mut C, Request) -> Result<Response, HttpError>,
}

impl<C> Route<C> {
    pub fn new(
        method: impl Into<String>,
        path: impl Into<String>,
        handler: fn(&mut C, Request) -> Result<Response, HttpError>,
    ) -> Self {
        Self {
            method: method.into(),
            path: path.into(),
            handler,
        }
    }
}
