use crate::raw_response::{ContentType, IntoRawResponse, RawResponse};

pub struct Response<T> {
    pub status_code: u16,
    pub headers: Vec<(String, String)>,
    pub body: Option<T>,
}

pub trait IntoBytes {
    fn into_bytes(self) -> Vec<u8>;
}

impl<T: IntoBytes + ContentType> IntoRawResponse for Response<T> {
    fn into_raw_response(self) -> RawResponse {
        RawResponse {
            status_code: self.status_code,
            headers: self.headers,
            content_type: self.body.as_ref().map(|_| T::content_type()),
            body: self.body.map(|body| body.into_bytes()),
        }
    }
}
