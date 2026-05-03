use crate::http::error::{HttpError, IntoError};

pub struct Response {
    pub body: Option<Vec<u8>>,
}

pub trait IntoResponse {
    fn into_response(self) -> Result<Response, impl IntoError>;
}

impl IntoResponse for Response {
    fn into_response(self) -> Result<Response, impl IntoError> {
        Ok::<Response, HttpError>(self)
    }
}
