pub struct Request {
    pub method: String,
    pub path: String,
    pub body: String,
}

impl Request {
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let request = std::str::from_utf8(bytes).ok()?;
        let mut lines = request.lines();

        let first_line = lines.next().unwrap_or("");
        let mut parts = first_line.split_whitespace();
        let method = parts.next().unwrap_or("").to_string();
        let path = parts.next().unwrap_or("").to_string();

        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
        }

        let body = lines.collect::<Vec<_>>().join("\n");

        Some(Request { method, path, body })
    }
}

pub trait FromRequest {
    fn from_request(request: Request) -> Self;
}

impl FromRequest for Request {
    fn from_request(request: Request) -> Self {
        request
    }
}

impl FromRequest for () {
    fn from_request(_: Request) -> Self {
        ()
    }
}
