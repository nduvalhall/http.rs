use crate::raw_response::{ContentType, IntoRawResponse, RawResponse};

pub struct HttpError {
    pub status_code: u16,
    pub headers: Vec<(String, String)>,
    pub detail: String,
}

impl HttpError {
    pub fn new(status_code: u16, detail: &str) -> HttpError {
        HttpError {
            status_code,
            headers: vec![],
            detail: detail.into(),
        }
    }
}

impl ContentType for HttpError {}

impl IntoRawResponse for HttpError {
    fn into_raw_response(self) -> RawResponse {
        RawResponse {
            status_code: self.status_code,
            headers: self.headers,
            content_type: Some(Self::content_type()),
            body: Some(self.detail.into()),
        }
    }
}
