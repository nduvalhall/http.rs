use std::fmt;

#[derive(Debug)]
pub enum Json {
    JsonNull,
    JsonBool(bool),
    JsonChar(char),
    JsonUint(u64),
    JsonInt(i64),
    JsonFloat(f64),
    JsonString(String),
    JsonList(Vec<Json>),
    JsonObject(Vec<(String, Json)>),
}

impl Json {
    pub fn to_string(&self) -> String {
        match self {
            Json::JsonNull => "null".to_string(),
            Json::JsonBool(b) => b.to_string(),
            Json::JsonChar(c) => c.to_string(),
            Json::JsonUint(u) => u.to_string(),
            Json::JsonInt(i) => i.to_string(),
            Json::JsonFloat(f) => f.to_string(),
            Json::JsonString(s) => format!("\"{}\"", s),
            Json::JsonList(v) => {
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
            Json::JsonObject(fields) => {
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

impl fmt::Display for Json {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}

pub trait IntoJson {
    fn to_json(self) -> Json;
}
