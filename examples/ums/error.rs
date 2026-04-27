use http::{IntoResponse, Response, StatusCode};

pub enum UMSError {
    UnknownError(String),
}

impl IntoResponse for UMSError {
    fn to_response(self) -> Response {
        match self {
            Self::UnknownError(message) => Response {
                status_code: StatusCode::InternalServerError,
                body: message.as_bytes().to_vec(),
            },
        }
    }
}
