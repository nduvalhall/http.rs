use crate::{IntoResponse, Method};

pub struct Request {
    pub method: Method,
    pub path: String,
    pub body: String,
}

impl Request {
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let request = std::str::from_utf8(bytes).ok()?;
        let mut lines = request.lines();

        let first_line = lines.next().unwrap_or("");
        let mut parts = first_line.split_whitespace();
        let method = parts.next().unwrap_or("");
        let path = parts.next().unwrap_or("").to_string();

        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
        }

        let body = lines.collect::<Vec<_>>().join("\n");

        Some(Request {
            method: Method::from_str(&method),
            path,
            body,
        })
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
