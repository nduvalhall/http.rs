use http::{IntoResponse, Response};

pub enum UMSError {
    UnknownError(String),
}

impl IntoResponse for UMSError {
    fn to_response(self) -> Response {
        match self {
            Self::UnknownError(message) => Response::InternalServerError(message),
        }
    }
}
