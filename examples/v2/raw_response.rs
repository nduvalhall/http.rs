pub struct RawResponse {
    pub status_code: u16,
    pub headers: Vec<(String, String)>,
    pub content_type: Option<&'static str>,
    pub body: Option<Vec<u8>>,
}

pub trait IntoRawResponse {
    fn into_raw_response(self) -> RawResponse;
}

pub trait ContentType {
    fn content_type() -> &'static str {
        "text/plain; charset=utf-8"
    }
}
