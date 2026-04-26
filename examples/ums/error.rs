use http::{Response, StatusCode, ToResponse};

pub enum Error {
    UnknownError(String),
}

impl ToResponse for Error {
    fn to_response(self) -> Response {
        match self {
            Self::UnknownError(message) => Response {
                status_code: StatusCode::InternalServerError,
                body: message.as_bytes().to_vec(),
            },
        }
    }
}
