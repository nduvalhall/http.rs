use std::fmt;

pub struct Response(ResponseInner);

enum ResponseInner {
    Ok(String),
    NoContent,
    Unauthorized,
    NotFound,
    MethodNotAllowed,
    InternalServerError(String),
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match &self.0 {
            ResponseInner::Ok(_) => "200 OK",
            ResponseInner::NoContent => "204 No Content",
            ResponseInner::Unauthorized => "401 Unauthorized",
            ResponseInner::NotFound => "404 Not Found",
            ResponseInner::MethodNotAllowed => "405 Method Not Allowed",
            ResponseInner::InternalServerError(_) => "500 Internal Server Error",
        };
        f.write_str(s)
    }
}

impl Response {
    pub fn ok(body: &str) -> Self {
        Response(ResponseInner::Ok(body.into()))
    }

    pub fn no_content() -> Self {
        Response(ResponseInner::NoContent)
    }

    pub fn unauthorized() -> Self {
        Response(ResponseInner::Unauthorized)
    }

    pub fn not_found() -> Self {
        Response(ResponseInner::NotFound)
    }

    pub fn method_not_allowed() -> Self {
        Response(ResponseInner::MethodNotAllowed)
    }

    pub fn internal_server_error(msg: &str) -> Self {
        Response(ResponseInner::InternalServerError(msg.into()))
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let status_code = format!("{}", &self);
        match self.0 {
            ResponseInner::Ok(body) | ResponseInner::InternalServerError(body) => {
                format!(
                    "HTTP/1.1 {}\r\nContent-Type: application/json; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
                    status_code,
                    body.len(),
                    body
                ).into_bytes()
            }
            ResponseInner::NoContent
            | ResponseInner::Unauthorized
            | ResponseInner::NotFound
            | ResponseInner::MethodNotAllowed => format!("HTTP/1.1 {}\r\n\r\n", self).into_bytes(),
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
        Response::ok(self)
    }
}

impl IntoResponse for String {
    fn to_response(self) -> Response {
        Response::ok(&self)
    }
}

impl IntoResponse for () {
    fn to_response(self) -> Response {
        Response::no_content()
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
            None => Err(Response::internal_server_error(msg)),
        }
    }
}

impl<T, E> OrInternalServerError<T> for Result<T, E> {
    fn or_internal_server_error(self, msg: &str) -> Result<T, Response> {
        match self {
            Ok(v) => Ok(v),
            Err(_) => Err(Response::internal_server_error(msg)),
        }
    }
}
