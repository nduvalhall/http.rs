use std::{collections::HashMap, io::Read, net::TcpStream};

use crate::{ContentType, HttpError};

pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

impl Request {
    pub fn from_stream(mut stream: TcpStream) -> (TcpStream, Result<Self, HttpError>) {
        let mut buf = vec![0u8; 16_384];

        let Ok(n) = stream.read(&mut buf) else {
            return (
                stream,
                Err(HttpError::new(500, "Failed to read from stream")),
            );
        };

        if n == 0 {
            return (
                stream,
                Err(HttpError::new(500, "Failed to read from stream")),
            );
        }

        let Some(header_end) = buf.windows(4).position(|w| w == b"\r\n\r\n") else {
            if n < buf.len() {
                return (
                    stream,
                    Err(HttpError::new(422, "Incomplete http packet received")),
                );
            } else {
                return (stream, Err(HttpError::new(413, "Http packet too large")));
            }
        };

        let Ok(header_str) = std::str::from_utf8(&buf[..header_end]) else {
            return (
                stream,
                Err(HttpError::new(422, "Header not utf8 compatible")),
            );
        };

        let mut lines = header_str.lines();

        let Some(first_line) = lines.next() else {
            return (
                stream,
                Err(HttpError::new(422, "Expected multiple lines in header")),
            );
        };

        let mut parts = first_line.split_whitespace();

        let Some(method) = parts.next() else {
            return (
                stream,
                Err(HttpError::new(422, "Expected method in first line")),
            );
        };

        let Some(path) = parts.next() else {
            return (
                stream,
                Err(HttpError::new(422, "Expected path in first line")),
            );
        };

        let mut headers = HashMap::new();
        for line in lines {
            if let Some((key, value)) = line.split_once(':') {
                headers.insert(key.trim().to_lowercase(), value.trim().to_string());
            }
        }

        let Some(content_length) = headers.get("content-length") else {
            return (
                stream,
                Ok(Self {
                    method: method.into(),
                    path: path.into(),
                    headers,
                    body: None,
                }),
            );
        };

        let Ok(content_length) = content_length.parse::<usize>() else {
            return (
                stream,
                Err(HttpError::new(422, "Content-Length must be an integer")),
            );
        };

        let buf_body_start = header_end + 4;
        let buf_body_len = n - buf_body_start;

        let mut body = Vec::with_capacity(content_length);
        body.extend_from_slice(&buf[buf_body_start..n]);

        if body.len() < content_length {
            body.resize(content_length, 0);
            let Ok(()) = stream.read_exact(&mut body[buf_body_len..]) else {
                return (
                    stream,
                    Err(HttpError::new(500, "Failed to read body from stream")),
                );
            };
        }

        (
            stream,
            Ok(Self {
                method: method.into(),
                path: path.into(),
                headers,
                body: Some(body),
            }),
        )
    }
}

pub trait FromRequest: Sized {
    fn from_request(request: &Request) -> Result<Self, HttpError>;
}

pub trait FromBytes: Sized {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self, HttpError>;
}
