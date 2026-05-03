use crate::http::error::{Error, IntoError};

pub struct HttpError();

impl IntoError for HttpError {
    fn into_error(self) -> super::Error {
        Error()
    }
}
