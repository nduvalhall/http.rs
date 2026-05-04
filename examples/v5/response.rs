use std::collections::HashMap;

pub enum ContentType {
    Json,
    Xml,
    Html,
    PlainText,
    FormUrlEncoded,
    MultipartFormData,
    OctetStream,
}

impl ContentType {
    pub fn to_string(&self) -> String {
        String::from(match self {
            ContentType::Json => "application/json",
            ContentType::Xml => "application/xml",
            ContentType::Html => "text/html",
            ContentType::PlainText => "text/plain",
            ContentType::FormUrlEncoded => "application/x-www-form-urlencoded",
            ContentType::MultipartFormData => "multipart/form-data",
            ContentType::OctetStream => "application/octet-stream",
        })
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "application/json" => Some(ContentType::Json),
            "application/xml" => Some(ContentType::Xml),
            "text/html" => Some(ContentType::Html),
            "text/plain" => Some(ContentType::PlainText),
            "application/x-www-form-urlencoded" => Some(ContentType::FormUrlEncoded),
            "multipart/form-data" => Some(ContentType::MultipartFormData),
            "application/octet-stream" => Some(ContentType::OctetStream),
            _ => None,
        }
    }
}

struct Body {
    content_type: ContentType,
    data: Vec<u8>,
}

impl Body {
    fn new(content_type: ContentType, data: Vec<u8>) -> Self {
        Body { content_type, data }
    }
}

pub struct Response {
    pub status: u16,
    pub headers: HashMap<String, String>,
    body: Option<Body>,
}

impl Response {
    pub fn new() -> Self {
        Self {
            status: 204,
            headers: HashMap::new(),
            body: None,
        }
    }

    pub fn status(self, status: u16) -> Self {
        Self { status, ..self }
    }

    pub fn header(self, header: impl Into<String>, value: impl Into<String>) -> Self {
        let mut headers = self.headers;
        headers.insert(header.into(), value.into());
        Self { headers, ..self }
    }

    pub fn headers(self, headers: HashMap<String, String>) -> Self {
        Self { headers, ..self }
    }

    pub fn body(self, content_type: ContentType, body: Vec<u8>) -> Self {
        Self {
            body: Some(Body::new(content_type, body)),
            ..self
        }
    }

    pub fn into_bytes(self) -> Result<Vec<u8>, &'static str> {
        let mut s = String::new();

        s.push_str(&format!("HTTP/1.1 {}\r\n", self.status));

        for (header, value) in self.headers {
            s.push_str(&format!("{}: {}\r\n", header, value));
        }

        if let Some(body) = self.body {
            s.push_str(&format!(
                "Content-Type: {}\r\n",
                body.content_type.to_string()
            ));
            s.push_str(&format!("Content-Length: {}\r\n", body.data.len()));
            s.push_str("\r\n");
            s.push_str(&String::from_utf8(body.data).map_err(|_| "Failed to parse response body")?);
        } else {
            s.push_str("\r\n");
        }

        Ok(s.into_bytes())
    }
}
