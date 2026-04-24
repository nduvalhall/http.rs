pub struct Request {
    pub method: String,
    pub path: String,
}

impl Request {
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let request = std::str::from_utf8(bytes).ok()?;
        let mut parts = request.split_whitespace();
        let method = parts.next()?.to_string();
        let path = parts.next()?.to_string();

        println!("{} {}", method, path);

        Some(Request { method, path })
    }
}
