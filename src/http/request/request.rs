use crate::http::error::{Error, HttpError, IntoError};

pub struct Request {
    pub body: Option<Vec<u8>>,
}

pub trait FromRequest: Sized {
    fn from_request(request: Request) -> Result<Self, impl IntoError>;
}

impl FromRequest for Request {
    fn from_request(request: Request) -> Result<Self, impl IntoError> {
        Ok::<Request, HttpError>(request)
    }
}

pub trait IntoRequest {
    fn into_request(self) -> Request;
}

impl IntoRequest for Request {
    fn into_request(self) -> Request {
        self
    }
}
