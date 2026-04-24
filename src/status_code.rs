use std::fmt;

pub enum StatusCode {
    Ok,
    NoContent,
    NotFound,
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Ok => "200 OK",
            Self::NoContent => "204 No Content",
            Self::NotFound => "404 Not Found",
        };
        f.write_str(s)
    }
}
