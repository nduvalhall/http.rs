use http::prelude::*;

pub enum UMSError {
    UnknownError(String),
}

impl IntoResponse for UMSError {
    fn to_response(self) -> Response {
        match self {
            Self::UnknownError(message) => Response::internal_server_error(&message),
        }
    }
}
