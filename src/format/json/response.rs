use std::collections::HashMap;

use crate::{ContentType, Error, IntoError, IntoJson, IntoResponse, Response};

pub struct JsonResponse<T: IntoJson> {
    status_code: u16,
    headers: HashMap<String, String>,
    body: T,
}

impl<T: IntoJson> JsonResponse<T> {
    pub fn new(body: T) -> Self {
        Self {
            status_code: 500,
            headers: HashMap::new(),
            body: body.into(),
        }
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

impl<T: IntoJson> IntoResponse for JsonResponse<T> {
    fn into_response(self) -> Result<Response, impl IntoError> {
        Ok::<Response, Error>(
            Response::new()
                .status_code(self.status_code)
                .headers(self.headers)
                .body(
                    ContentType::Json,
                    self.body.into_json().into_string().into_bytes(),
                ),
        )
    }
}
