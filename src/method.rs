#[derive(PartialEq, Eq, Debug)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Options,
    Unknown(String),
}

impl Method {
    pub fn from_str(method: &str) -> Method {
        match method {
            "GET" => Method::Get,
            "POST" => Method::Post,
            "PUT" => Method::Put,
            "DELETE" => Method::Delete,
            "OPTIONS" => Method::Options,
            other => Method::Unknown(other.to_string()),
        }
    }
}
