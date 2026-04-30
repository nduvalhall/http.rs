use std::collections::HashMap;

use crate::HttpError;

pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

impl Request {
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let header_end = bytes.windows(4).position(|w| w == b"\r\n\r\n")?;
        let body_start = header_end + 4;

        let header_str = std::str::from_utf8(&bytes[..header_end]).ok()?;
        let mut lines = header_str.lines();

        let first_line = lines.next()?;
        let mut parts = first_line.split_whitespace();
        let method = parts.next()?;
        let path = parts.next()?.to_string();

        let mut headers = HashMap::new();
        for line in lines {
            if let Some((key, value)) = line.split_once(':') {
                headers.insert(key.trim().to_lowercase(), value.trim().to_string());
            }
        }

        let body = if let Some(len) = headers
            .get("content-length")
            .and_then(|v| v.parse::<usize>().ok())
        {
            Some(bytes.get(body_start..body_start + len)?.into())
        } else {
            None
        };

        let req = Self {
            method: method.into(),
            path,
            body,
            headers,
        };

        Some(req)
    }
}

pub trait FromRequest: Sized {
    fn from_request(request: &Request) -> Result<Self, HttpError>;
}

pub trait FromBytes: Sized {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self, HttpError>;
}
