use std::fmt;

pub enum StatusCode {
    Ok,
    NoContent,
    Unauthorized,
    NotFound,
    InternalServerError,
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Ok => "200 OK",
            Self::NoContent => "204 No Content",
            Self::Unauthorized => "401 Unauthorized",
            Self::NotFound => "404 Not Found",
            Self::InternalServerError => "500 Internal Server Error",
        };
        f.write_str(s)
    }
}
