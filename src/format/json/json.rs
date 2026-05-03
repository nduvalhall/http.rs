use std::collections::HashMap;

#[derive(Debug)]
pub struct JsonObject(pub HashMap<String, Json>);

#[derive(Debug)]
pub enum Json {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(JsonObject),
}

impl Json {
    pub fn null() -> Self {
        Json::Null
    }

    pub fn bool(b: bool) -> Self {
        Json::Bool(b)
    }

    pub fn number(s: impl Into<f64>) -> Self {
        Json::Number(s.into())
    }

    pub fn string(s: impl Into<String>) -> Self {
        Json::String(s.into())
    }

    pub fn array(items: Vec<Json>) -> Self {
        Json::Array(items)
    }

    pub fn object(fields: Vec<(impl Into<String>, Json)>) -> Self {
        Json::Object(JsonObject(
            fields.into_iter().map(|(k, j)| (k.into(), j)).collect(),
        ))
    }

    pub fn as_object(&self) -> Result<&JsonObject, JsonError> {
        match self {
            Json::Object(obj) => Ok(obj),
            _ => Err(JsonError("expected object".into())),
        }
    }

    pub fn into_object(self) -> Result<JsonObject, JsonError> {
        match self {
            Json::Object(obj) => Ok(obj),
            _ => Err(JsonError("expected object".into())),
        }
    }

    pub fn as_string(&self) -> Result<String, JsonError> {
        match self {
            Json::String(s) => Ok(s.clone()),
            _ => Err(JsonError("expected string".into())),
        }
    }

    pub fn as_number<T: FromF64>(&self) -> Result<T, JsonError> {
        match self {
            Json::Number(n) => Ok(T::from_f64(*n)),
            _ => Err(JsonError("expected number".into())),
        }
    }

    pub fn as_array(&self) -> Result<&Vec<Json>, JsonError> {
        match self {
            Json::Array(v) => Ok(v),
            _ => Err(JsonError("expected array".into())),
        }
    }

    pub fn as_bool(&self) -> Result<bool, JsonError> {
        match self {
            Json::Bool(b) => Ok(*b),
            _ => Err(JsonError("expected bool".into())),
        }
    }

    pub fn as_null(&self) -> Result<(), JsonError> {
        match self {
            Json::Null => Ok(()),
            _ => Err(JsonError("expected null".into())),
        }
    }

    pub fn into_string(&self) -> String {
        match self {
            Json::Null => "null".to_string(),
            Json::Bool(b) => b.to_string(),
            Json::Number(f) => format!("{}", f),
            Json::String(s) => format!("\"{}\"", s),
            Json::Array(v) => {
                let mut iter = v.iter().peekable();
                let mut s = String::from("[");
                while let Some(j) = iter.next() {
                    s.push_str(&j.into_string());
                    if iter.peek().is_some() {
                        s.push(',')
                    }
                }
                s.push(']');
                s
            }
            Json::Object(obj) => {
                let mut iter = obj.0.iter().peekable();
                let mut s = String::from("{");
                while let Some((k, j)) = iter.next() {
                    s.push_str(&format!("\"{}\":{}", k, j.into_string()));
                    if iter.peek().is_some() {
                        s.push(',')
                    }
                }
                s.push('}');
                s
            }
        }
    }
}

pub trait FromF64 {
    fn from_f64(n: f64) -> Self;
}

impl FromF64 for f64 {
    fn from_f64(n: f64) -> Self {
        n
    }
}
impl FromF64 for f32 {
    fn from_f64(n: f64) -> Self {
        n as f32
    }
}
impl FromF64 for i8 {
    fn from_f64(n: f64) -> Self {
        n as i8
    }
}
impl FromF64 for i16 {
    fn from_f64(n: f64) -> Self {
        n as i16
    }
}
impl FromF64 for i32 {
    fn from_f64(n: f64) -> Self {
        n as i32
    }
}
impl FromF64 for i64 {
    fn from_f64(n: f64) -> Self {
        n as i64
    }
}
impl FromF64 for u8 {
    fn from_f64(n: f64) -> Self {
        n as u8
    }
}
impl FromF64 for u16 {
    fn from_f64(n: f64) -> Self {
        n as u16
    }
}
impl FromF64 for u32 {
    fn from_f64(n: f64) -> Self {
        n as u32
    }
}
impl FromF64 for u64 {
    fn from_f64(n: f64) -> Self {
        n as u64
    }
}
impl FromF64 for usize {
    fn from_f64(n: f64) -> Self {
        n as usize
    }
}

pub trait IntoJson {
    fn into_json(&self) -> Json;
}

#[derive(Debug)]
pub struct JsonError(pub String);

impl std::fmt::Display for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub trait FromJson: Sized {
    fn from_json(json: Json) -> Result<Self, JsonError>;
}

impl TryFrom<Json> for JsonObject {
    type Error = JsonError;

    fn try_from(json: Json) -> Result<Self, JsonError> {
        match json {
            Json::Object(obj) => Ok(obj),
            _ => Err(JsonError("expected object".into())),
        }
    }
}

impl JsonObject {
    pub fn field(&self, key: &str) -> Option<&Json> {
        self.0.get(key)
    }

    pub fn get<T: FromJson>(&mut self, key: &str) -> Result<T, JsonError> {
        let json = self
            .0
            .remove(key)
            .ok_or_else(|| JsonError(format!("missing field: {key}")))?;
        T::from_json(json)
    }

    pub fn get_opt<T: FromJson>(&mut self, key: &str) -> Result<Option<T>, JsonError> {
        match self.0.remove(key) {
            None | Some(Json::Null) => Ok(None),
            Some(json) => T::from_json(json).map(Some),
        }
    }
}

impl FromJson for String {
    fn from_json(json: Json) -> Result<Self, JsonError> {
        json.as_string()
    }
}

impl FromJson for i32 {
    fn from_json(json: Json) -> Result<Self, JsonError> {
        json.as_number()
    }
}

impl FromJson for u8 {
    fn from_json(json: Json) -> Result<Self, JsonError> {
        json.as_number()
    }
}

impl FromJson for bool {
    fn from_json(json: Json) -> Result<Self, JsonError> {
        json.as_bool()
    }
}

impl<T: FromJson> FromJson for Vec<T> {
    fn from_json(json: Json) -> Result<Self, JsonError> {
        match json {
            Json::Array(items) => items.into_iter().map(T::from_json).collect(),
            _ => Err(JsonError("expected array".into())),
        }
    }
}
