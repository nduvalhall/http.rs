use std::fmt;

use crate::{IntoJson, IntoResponse, Json, JsonValue, Response};

#[derive(Debug)]
pub struct HttpError {
    status_code: u16,
    detail: String,
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = format!(
            "HttpError(status_code: {}, detail: {})",
            self.status_code, self.detail
        );
        f.write_str(&s)
    }
}

impl IntoJson for HttpError {
    fn into_json(self) -> JsonValue {
        JsonValue::JsonObject(vec![("detail".into(), JsonValue::JsonString(self.detail))])
    }
}

impl HttpError {
    pub fn new(status_code: u16, detail: &str) -> HttpError {
        eprintln!("{}", detail);
        HttpError {
            status_code,
            detail: detail.into(),
        }
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        Response::new()
            .status_code(self.status_code)
            .body(Json(self))
    }
}
