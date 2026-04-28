use crate::{Method, Response};

use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub body: String,
    headers: HashMap<String, String>,
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
            let body_bytes = bytes.get(body_start..body_start + len)?;
            std::str::from_utf8(body_bytes).ok()?.to_string()
        } else {
            String::new()
        };

        let req = Request {
            method: Method::from_str(&method),
            path,
            body,
            headers,
        };

        Some(req)
    }

    pub fn get_header(&self, name: &str) -> Option<&str> {
        self.headers.get(&name.to_lowercase()).map(|v| v.as_str())
    }
}

pub trait FromRequest: Sized {
    fn from_request(request: Request) -> Result<Self, Response>;
}

impl FromRequest for Request {
    fn from_request(request: Request) -> Result<Self, Response> {
        Ok(request)
    }
}

impl FromRequest for () {
    fn from_request(_: Request) -> Result<Self, Response> {
        Ok(())
    }
}
