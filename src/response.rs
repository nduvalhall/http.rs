use std::{collections::HashMap, fmt};

use crate::body::Body;

/// MIME content types recognized by the framework.
#[derive(Debug)]
pub enum ContentType {
    /// `application/json`
    Json,
    /// `application/xml`
    Xml,
    /// `text/html`
    Html,
    /// `text/plain`
    PlainText,
    /// `application/x-www-form-urlencoded`
    FormUrlEncoded,
    /// `multipart/form-data`
    MultipartFormData,
    /// `application/octet-stream`
    OctetStream,
}

impl ContentType {
    /// Returns the MIME type string (e.g. `"application/json"`).
    pub fn to_str(&self) -> &'static str {
        match self {
            ContentType::Json => "application/json",
            ContentType::Xml => "application/xml",
            ContentType::Html => "text/html",
            ContentType::PlainText => "text/plain",
            ContentType::FormUrlEncoded => "application/x-www-form-urlencoded",
            ContentType::MultipartFormData => "multipart/form-data",
            ContentType::OctetStream => "application/octet-stream",
        }
    }

    /// Parses an exact MIME type string. Returns `None` for unrecognized types.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "application/json" => Some(ContentType::Json),
            "application/xml" => Some(ContentType::Xml),
            "text/html" => Some(ContentType::Html),
            "text/plain" => Some(ContentType::PlainText),
            "application/x-www-form-urlencoded" => Some(ContentType::FormUrlEncoded),
            "multipart/form-data" => Some(ContentType::MultipartFormData),
            "application/octet-stream" => Some(ContentType::OctetStream),
            _ => None,
        }
    }
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.to_str();
        f.write_str(s)
    }
}

/// An outgoing HTTP response.
pub struct Response {
    /// HTTP status code.
    pub status: u16,
    /// Custom response headers (excluding `Content-Type` and `Content-Length`, which are set automatically).
    pub headers: HashMap<String, String>,
    body: Option<Body>,
}

impl Response {
    /// Creates a response with status `204 No Content` and no body.
    pub fn new() -> Self {
        Self {
            status: 204,
            headers: HashMap::new(),
            body: None,
        }
    }

    /// Sets the HTTP status code.
    pub fn status(self, status: u16) -> Self {
        Self { status, ..self }
    }

    /// Adds a single response header.
    pub fn header(self, header: impl Into<String>, value: impl Into<String>) -> Self {
        let mut headers = self.headers;
        headers.insert(header.into(), value.into());
        Self { headers, ..self }
    }

    /// Replaces all custom headers with the provided map.
    pub fn headers(self, headers: HashMap<String, String>) -> Self {
        Self { headers, ..self }
    }

    /// Sets the response body; `Content-Type` and `Content-Length` are added automatically.
    pub fn body(self, content_type: ContentType, body: Vec<u8>) -> Self {
        Self {
            body: Some(Body::new(content_type, body)),
            ..self
        }
    }

    /// Serializes the response to raw HTTP/1.1 bytes for writing to a TCP stream.
    pub fn into_bytes(mut self) -> Result<Vec<u8>, &'static str> {
        let mut s = String::new();

        s.push_str(&format!("HTTP/1.1 {}\r\n", self.status));

        self.headers
            .iter_mut()
            .for_each(|(_, v)| *v = v.to_lowercase());

        let _ = self.headers.remove("content-type");
        let _ = self.headers.remove("content-length");

        for (header, value) in self.headers {
            s.push_str(&format!("{}: {}\r\n", header, value));
        }

        return match self.body {
            Some(b) => {
                s.push_str(&format!("Content-Type: {}\r\n", b.content_type.to_string()));
                s.push_str(&format!("Content-Length: {}\r\n\r\n", b.data.len()));
                let mut b_ = s.into_bytes();
                b_.extend(b.data);
                Ok(b_)
            }
            None => {
                s.push_str("\r\n");
                Ok(s.into_bytes())
            }
        };
    }
}
