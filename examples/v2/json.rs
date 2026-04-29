use crate::{
    http_error::HttpError, raw_response::ContentType, request::FromBytes, response::IntoBytes,
};

pub enum JsonValue {
    JsonString(String),
}

pub trait FromJson {
    fn from_json(json: JsonValue) -> Self;
}

pub trait IntoJson {
    fn into_json(self) -> JsonValue;
}

impl FromBytes for JsonValue {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self, HttpError> {
        Ok(JsonValue::JsonString("".into()))
    }
}

impl IntoBytes for JsonValue {
    fn into_bytes(self) -> Vec<u8> {
        match self {
            JsonValue::JsonString(s) => s.into(),
        }
    }
}

pub struct Json<T>(pub T);

impl<T: FromJson> FromBytes for Json<T> {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self, HttpError> {
        Ok(Json(T::from_json(JsonValue::from_bytes(bytes)?)))
    }
}

impl<T: IntoJson> IntoBytes for Json<T> {
    fn into_bytes(self) -> Vec<u8> {
        let Json(body) = self;
        body.into_json().into_bytes()
    }
}

impl<T> ContentType for Json<T> {
    fn content_type() -> &'static str {
        "application/json; charset=utf-8"
    }
}
