pub enum StatusCode {
    NoContent,
}

impl StatusCode {
    fn to_string(self) -> String {
        match self {
            Self::NoContent => String::from("204 No Content"),
        }
    }
}

pub struct Response {
    pub status_code: StatusCode,
}

impl Response {
    pub fn to_string(self) -> String {
        format!("HTTP/1.1 {}", self.status_code.to_string())
    }
}
