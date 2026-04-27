use crate::{IntoResponse, Method};
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
        let request = std::str::from_utf8(bytes).ok()?;
        let mut lines = request.lines();

        let first_line = lines.next().unwrap_or("");
        let mut parts = first_line.split_whitespace();
        let method = parts.next().unwrap_or("");
        let path = parts.next().unwrap_or("").to_string();

        let mut headers = HashMap::new();
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            if let Some((key, value)) = line.split_once(':') {
                headers.insert(key.trim().to_lowercase(), value.trim().to_string());
            }
        }

        let body = lines.collect::<Vec<_>>().join("\n");

        let req = Request {
            method: Method::from_str(&method),
            path,
            body,
            headers,
        };

        println!("{:?}", req);

        Some(req)
    }

    pub fn get_header(&self, name: &str) -> Option<&str> {
        self.headers.get(&name.to_lowercase()).map(|v| v.as_str())
    }
}

pub trait FromRequest: Sized {
    type Error: IntoResponse;
    fn from_request(request: Request) -> Result<Self, Self::Error>;
}

impl FromRequest for Request {
    type Error = ();
    fn from_request(request: Request) -> Result<Self, Self::Error> {
        Ok(request)
    }
}

impl FromRequest for () {
    type Error = ();
    fn from_request(_: Request) -> Result<Self, Self::Error> {
        Ok(())
    }
}
