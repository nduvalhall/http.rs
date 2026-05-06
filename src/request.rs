use std::{collections::HashMap, io::Read};

use crate::{Body, ContentType};

pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Body>,
}

impl Request {
    fn new(method: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            method: method.into(),
            path: path.into(),
            headers: HashMap::new(),
            body: None,
        }
    }

    fn body(self, content_type: ContentType, body: Vec<u8>) -> Self {
        Self {
            body: Some(Body::new(content_type, body)),
            ..self
        }
    }

    pub fn from_reader<R: Read>(reader: &mut R) -> Result<Self, &'static str> {
        let mut buf = vec![0u8; 16_384];

        let Ok(n) = reader.read(&mut buf) else {
            return Err("Failed to read from reader");
        };

        if n == 0 {
            return Err("Reader closed");
        }

        let Some(header_end) = buf.windows(4).position(|w| w == b"\r\n\r\n") else {
            if n < buf.len() {
                return Err("Incomplete http packet received");
            } else {
                return Err("Http header too large");
            }
        };

        let Ok(header_str) = std::str::from_utf8(&buf[..header_end]) else {
            return Err("Header not utf8 compatible");
        };

        let mut lines = header_str.lines();

        let Some(first_line) = lines.next() else {
            return Err("Expected multiple lines in header");
        };

        let mut parts = first_line.split_whitespace();

        let Some(method) = parts.next() else {
            return Err("Expected method in first line");
        };

        let Some(path) = parts.next() else {
            return Err("Expected path in first line");
        };

        let mut headers = HashMap::new();
        for line in lines {
            if let Some((key, value)) = line.split_once(':') {
                headers.insert(key.trim().to_lowercase(), value.trim().to_string());
            }
        }

        let Some(content_length) = headers.get("content-length") else {
            return Ok(Self::new(method, path));
        };

        let Ok(content_length) = content_length.parse::<usize>() else {
            return Err("Content-Length must be an integer");
        };

        let content_type = match headers.get("content-type") {
            Some(s) => match ContentType::from_str(s) {
                Some(c) => c,
                None => return Err("Unknown Content-Type"),
            },
            None => ContentType::PlainText,
        };

        let buf_body_start = header_end + 4;
        let buf_body_len = n - buf_body_start;

        let mut body = Vec::with_capacity(content_length);
        body.extend_from_slice(&buf[buf_body_start..n]);

        if body.len() < content_length {
            body.resize(content_length, 0);
            let Ok(()) = reader.read_exact(&mut body[buf_body_len..]) else {
                return Err("Failed to read body from reader");
            };
        }

        Ok(Self::new(method, path).body(content_type, body))
    }
}
