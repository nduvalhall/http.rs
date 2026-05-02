use crate::{ContentType, FromBytes, HttpError, IntoBytes};

mod lexer;
mod parser;

pub enum JsonValue {
    JsonNull,
    JsonBool(bool),
    JsonChar(char),
    JsonUint(u64),
    JsonInt(i64),
    JsonFloat(f64),
    JsonString(String),
    JsonList(Vec<JsonValue>),
    JsonObject(Vec<(String, JsonValue)>),
}

impl JsonValue {
    pub fn to_string(&self) -> String {
        match self {
            JsonValue::JsonNull => "null".to_string(),
            JsonValue::JsonBool(b) => b.to_string(),
            JsonValue::JsonChar(c) => format!("\"{}\"", c),
            JsonValue::JsonUint(u) => u.to_string(),
            JsonValue::JsonInt(i) => i.to_string(),
            JsonValue::JsonFloat(f) => f.to_string(),
            JsonValue::JsonString(s) => format!("\"{}\"", s),
            JsonValue::JsonList(v) => {
                let mut res = String::from("[");
                let mut iter = v.iter().peekable();

                loop {
                    if let Some(json_value) = iter.next() {
                        res.push_str(&json_value.to_string());

                        if iter.peek().is_some() {
                            res.push_str(", ");
                        }
                    } else {
                        break;
                    }
                }

                res.push(']');
                res
            }
            JsonValue::JsonObject(fields) => {
                let mut res = String::from("{");
                let mut iter = fields.iter().peekable();

                loop {
                    if let Some((key, json_value)) = iter.next() {
                        let s = format!("\"{}\": {}", key, json_value.to_string());
                        res.push_str(&s);

                        if iter.peek().is_some() {
                            res.push_str(", ");
                        }
                    } else {
                        break;
                    }
                }
                res.push('}');
                res
            }
        }
    }
}

pub trait FromJson {
    fn from_json(json: JsonValue) -> Self;
}

pub trait IntoJson {
    fn into_json(self) -> JsonValue;
}

impl FromBytes for JsonValue {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self, HttpError> {
        let Ok(s) = String::from_utf8(bytes) else {
            return Err(HttpError::new(400, "request body is not valid utf-8"));
        };
        parser::parse(&s).map_err(|e| HttpError::new(400, &e))
    }
}

impl IntoBytes for JsonValue {
    fn into_bytes(self) -> Vec<u8> {
        self.to_string().into_bytes()
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
    fn content_type(&self) -> &'static str {
        "application/json; charset=utf-8"
    }
}

impl IntoJson for String {
    fn into_json(self) -> JsonValue {
        JsonValue::JsonString(self)
    }
}
