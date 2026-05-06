use crate::{http_error::HttpError, request::Request};

pub struct Middleware<C> {
    pub method: String,
    pub path: String,
    pub handler: fn(&mut C, Request) -> Result<Request, HttpError>,
}

impl<C> Middleware<C> {
    pub fn new(
        method: impl Into<String>,
        path: impl Into<String>,
        f: fn(&mut C, Request) -> Result<Request, HttpError>,
    ) -> Self {
        Self {
            method: method.into(),
            path: path.into(),
            handler: f,
        }
    }
}
