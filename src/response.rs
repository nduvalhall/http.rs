use crate::HttpError;

#[derive(Debug)]
pub struct Response {
    status_code: u16,
    headers: Vec<(String, String)>,
    body: Option<(&'static str, Vec<u8>)>,
}

pub trait IntoBytes {
    fn into_bytes(self) -> Vec<u8>;
}

pub trait ContentType {
    fn content_type(&self) -> &'static str {
        "text/plain; charset=utf-8"
    }
}

pub trait IntoResponse {
    fn into_response(self) -> Response;
}

impl Response {
    pub fn new() -> Self {
        Self {
            status_code: 200,
            headers: Vec::new(),
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
        headers.push((header.into(), value.into()));
        Self { headers, ..self }
    }

    pub fn body(self, body: impl IntoBytes + ContentType) -> Self {
        let content_type = body.content_type();
        let body = body.into_bytes();
        Self {
            body: Some((content_type, body)),
            ..self
        }
    }

    pub fn ok(body: impl IntoBytes + ContentType) -> Self {
        Self::new().body(body)
    }

    pub fn created(body: impl IntoBytes + ContentType) -> Self {
        Self::new().status_code(201).body(body)
    }

    pub fn no_content() -> Self {
        Self::new().status_code(204)
    }

    pub fn bad_request(body: impl IntoBytes + ContentType) -> Self {
        Self::new().status_code(400).body(body)
    }

    pub fn unauthorized() -> Self {
        Self::new().status_code(401)
    }

    pub fn forbidden(body: impl IntoBytes + ContentType) -> Self {
        Self::new().status_code(403).body(body)
    }

    pub fn not_found(body: impl IntoBytes + ContentType) -> Self {
        Self::new().status_code(404).body(body)
    }

    pub fn internal_server_error(body: impl IntoBytes + ContentType) -> Self {
        Self::new().status_code(500).body(body)
    }

    pub fn into_bytes(self) -> Result<Vec<u8>, HttpError> {
        let mut s = String::new();

        s.push_str(&format!("HTTP/1.1 {}\r\n", self.status_code));

        for (header, value) in self.headers {
            s.push_str(&format!("{}: {}\r\n", header, value));
        }

        if let Some((content_type, body)) = self.body {
            s.push_str(&format!("Content-Type: {}\r\n", content_type));
            s.push_str(&format!("Content-Length: {}\r\n", body.len()));
            s.push_str("\r\n");
            s.push_str(
                &String::from_utf8(body)
                    .map_err(|_| HttpError::new(500, "Failed to parse response body"))?,
            );
        } else {
            s.push_str("\r\n");
        }

        Ok(s.into_bytes())
    }
}

impl IntoBytes for String {
    fn into_bytes(self) -> Vec<u8> {
        self.into()
    }
}

impl ContentType for String {}

impl IntoBytes for &str {
    fn into_bytes(self) -> Vec<u8> {
        self.into()
    }
}

impl ContentType for &str {}
