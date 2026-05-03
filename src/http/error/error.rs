use std::{collections::HashMap, fmt};

use crate::{ContentType, IntoJson, IntoResponse, Json, Response};

#[derive(Debug)]
pub struct Error {
    status_code: u16,
    headers: HashMap<String, String>,
    detail: String,
}

impl Error {
    pub fn new(status_code: u16, detail: impl Into<String>) -> Self {
        Self {
            status_code,
            headers: HashMap::new(),
            detail: detail.into(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait IntoError {
    fn into_error(self) -> Error;
    fn get_status_code(&self) -> u16;

    fn get_detail(&self) -> &str;
}

impl IntoError for Error {
    fn into_error(self) -> Error {
        self
    }
    fn get_status_code(&self) -> u16 {
        self.status_code
    }
    fn get_detail(&self) -> &str {
        &self.detail
    }
}

impl IntoJson for Error {
    fn into_json(&self) -> crate::Json {
        Json::object(vec![("detail", Json::string(&self.detail))])
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Result<Response, impl IntoError> {
        Ok::<Response, Error>(
            Response::new()
                .status_code(self.status_code)
                .headers(self.headers)
                .body(ContentType::PlainText, self.detail.into_bytes()),
        )
    }
}
