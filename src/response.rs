use crate::status_code::StatusCode;

pub struct Response {
    pub status_code: StatusCode,
    pub body: Vec<u8>,
}

impl Response {
    pub fn new(status_code: StatusCode) -> Self {
        Response {
            status_code,
            body: Vec::new(),
        }
    }

    pub fn with_body(status_code: StatusCode, body: Vec<u8>) -> Self {
        Response { status_code, body }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let header = match self.status_code {
            StatusCode::Ok => {
                format!(
                    "HTTP/1.1 {}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n",
                    self.status_code,
                    self.body.len()
                )
            }
            StatusCode::NoContent | StatusCode::NotFound => {
                format!("HTTP/1.1 {}\r\n\r\n", self.status_code)
            }
        };

        let mut bytes = header.into_bytes();
        bytes.extend_from_slice(&self.body);
        bytes
    }
}
