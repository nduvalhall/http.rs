use std::collections::HashMap;

use crate::{ContentType, IntoError, http::error::Error};

struct Body {
    content_type: String,
    data: Vec<u8>,
}

pub struct Response {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    body: Option<Body>,
}

impl Response {
    pub fn new() -> Self {
        Self {
            status_code: 200,
            headers: HashMap::new(),
            body: None,
        }
    }
    pub fn status_code(self, status_code: u16) -> Self {
        Self {
            status_code,
            ..self
        }
    }

    pub fn header(self, header: &str, value: &str) -> Self {
        let mut headers = self.headers;
        headers.insert(header.into(), value.into());
        Self { headers, ..self }
    }

    pub fn headers(self, headers: HashMap<String, String>) -> Self {
        Self { headers, ..self }
    }

    pub fn body(self, content_type: ContentType, body: Vec<u8>) -> Self {
        Self {
            body: Some(Body {
                content_type: content_type.to_string(),
                data: body,
            }),
            ..self
        }
    }
    pub fn into_bytes(self) -> Result<Vec<u8>, Error> {
        let mut s = String::new();

        s.push_str(&format!("HTTP/1.1 {}\r\n", self.status_code));

        for (header, value) in self.headers {
            s.push_str(&format!("{}: {}\r\n", header, value));
        }

        if let Some(body) = self.body {
            s.push_str(&format!("Content-Type: {}\r\n", body.content_type));
            s.push_str(&format!("Content-Length: {}\r\n", body.data.len()));
            s.push_str("\r\n");
            s.push_str(
                &String::from_utf8(body.data)
                    .map_err(|_| Error::new(500, "Failed to parse response body"))?,
            );
        } else {
            s.push_str("\r\n");
        }

        Ok(s.into_bytes())
    }
}

pub trait IntoResponse {
    fn into_response(self) -> Result<Response, impl IntoError>;
}

impl IntoResponse for Response {
    fn into_response(self) -> Result<Response, impl IntoError> {
        Ok::<Response, Error>(self)
    }
}

pub trait IntoBytes {
    fn into_bytes(self) -> Vec<u8>;
}
