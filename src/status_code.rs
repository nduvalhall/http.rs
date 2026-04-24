pub enum StatusCode {
    Ok,
    NoContent,
    NotFound,
}

impl StatusCode {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Ok => "200 OK",
            Self::NoContent => "204 No Content",
            Self::NotFound => "404 Not Found",
        }
    }
}
