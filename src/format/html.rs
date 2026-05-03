use std::{collections::HashMap, fs::File, io::Read};

use crate::{ContentType, Error, IntoError, IntoResponse, Response};

pub struct HtmlResponse {
    status_code: u16,
    headers: HashMap<String, String>,
    body: String,
}

impl HtmlResponse {
    pub fn new(body: impl Into<String>) -> Self {
        Self {
            status_code: 200,
            headers: HashMap::new(),
            body: body.into(),
        }
    }

    pub fn from_file(filepath: &str) -> Result<Self, ()> {
        let mut file = File::open(filepath).map_err(|_| ())?;
        let mut content = String::new();
        file.read_to_string(&mut content).map_err(|_| ())?;
        Ok(Self::new(content))
    }

    pub fn status_code(self, status_code: u16) -> Self {
        Self {
            status_code,
            ..self
        }
    }

    pub fn header(self, header: &str, value: &str) -> Self {
        let mut headers = self.headers;
        headers.insert(header.into(), value.into());
        Self { headers, ..self }
    }
}

impl IntoResponse for HtmlResponse {
    fn into_response(self) -> Result<Response, impl IntoError> {
        Ok::<Response, Error>(
            Response::new()
                .status_code(self.status_code)
                .headers(self.headers)
                .body(ContentType::Html, self.body.into_bytes()),
        )
    }
}
