use std::fmt;

pub enum Response {
    Ok(String),
    NoContent,
    Unauthorized,
    NotFound,
    InternalServerError(String),
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Ok(_) => "200 OK",
            Self::NoContent => "204 No Content",
            Self::Unauthorized => "401 Unauthorized",
            Self::NotFound => "404 Not Found",
            Self::InternalServerError(_) => "500 Internal Server Error",
        };
        f.write_str(s)
    }
}

impl Response {
    pub fn to_bytes(&self) -> Vec<u8> {
        let status_code = format!("{}", &self);
        match self {
            Self::Ok(body) | Self::InternalServerError(body) => {
                format!(
                    "HTTP/1.1 {}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
                    status_code,
                    body.len(),
                    body
                ).into_bytes()
            }
            Self::NoContent | Self::Unauthorized | Self::NotFound => {
                format!("HTTP/1.1 {}\r\n\r\n", self).into_bytes()
            }
        }
    }
}

pub trait IntoResponse {
    fn to_response(self) -> Response;
}

impl IntoResponse for Response {
    fn to_response(self) -> Response {
        self
    }
}

impl IntoResponse for &str {
    fn to_response(self) -> Response {
        Response::Ok(self.to_string())
    }
}

impl IntoResponse for String {
    fn to_response(self) -> Response {
        Response::Ok(self)
    }
}

impl IntoResponse for () {
    fn to_response(self) -> Response {
        Response::NoContent
    }
}

impl<T: IntoResponse, E: IntoResponse> IntoResponse for Result<T, E> {
    fn to_response(self) -> Response {
        match self {
            Ok(val) => val.to_response(),
            Err(err) => err.to_response(),
        }
    }
}

pub trait OrInternalServerError<T> {
    fn or_internal_server_error(self, msg: &str) -> Result<T, Response>;
}

impl<T> OrInternalServerError<T> for Option<T> {
    fn or_internal_server_error(self, msg: &str) -> Result<T, Response> {
        match self {
            Some(v) => Ok(v),
            None => Err(Response::InternalServerError(msg.to_string())),
        }
    }
}

impl<T, E> OrInternalServerError<T> for Result<T, E> {
    fn or_internal_server_error(self, msg: &str) -> Result<T, Response> {
        match self {
            Ok(v) => Ok(v),
            Err(_) => Err(Response::InternalServerError(msg.to_string())),
        }
    }
}
