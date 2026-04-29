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
        let mut headers = self.headers;

        let content_type = T::content_type();
        headers.push(("Content-Type".into(), content_type.into()));

        RawResponse {
            status_code: self.status_code,
            headers: headers,
            body: self.body.map(|body| body.into_bytes()),
        }
    }
}
