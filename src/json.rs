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

pub trait IntoJsonObject {
    fn to_json_boxed(self: Box<Self>) -> Json;
}

impl<T: IntoJson> IntoJsonObject for T {
    fn to_json_boxed(self: Box<Self>) -> Json {
        (*self).to_json()
    }
}

impl IntoJson for Json {
    fn to_json(self) -> Json {
        self
    }
}

impl IntoJson for &str {
    fn to_json(self) -> Json {
        Json::JsonString(self.to_string())
    }
}

impl IntoJson for String {
    fn to_json(self) -> Json {
        Json::JsonString(self)
    }
}

impl IntoJson for () {
    fn to_json(self) -> Json {
        Json::JsonNull
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null_to_string() {
        assert_eq!(Json::JsonNull.to_string(), "null");
    }

    #[test]
    fn bool_to_string() {
        assert_eq!(Json::JsonBool(true).to_string(), "true");
        assert_eq!(Json::JsonBool(false).to_string(), "false");
    }

    #[test]
    fn char_to_string() {
        assert_eq!(Json::JsonChar('a').to_string(), "a");
    }

    #[test]
    fn uint_to_string() {
        assert_eq!(Json::JsonUint(42).to_string(), "42");
        assert_eq!(Json::JsonUint(0).to_string(), "0");
    }

    #[test]
    fn int_to_string() {
        assert_eq!(Json::JsonInt(-7).to_string(), "-7");
        assert_eq!(Json::JsonInt(0).to_string(), "0");
    }

    #[test]
    fn float_to_string() {
        assert_eq!(Json::JsonFloat(3.14).to_string(), "3.14");
    }

    #[test]
    fn string_to_string() {
        assert_eq!(Json::JsonString("hello".to_string()).to_string(), "\"hello\"");
    }

    #[test]
    fn empty_list_to_string() {
        assert_eq!(Json::JsonList(vec![]).to_string(), "[]");
    }

    #[test]
    fn list_to_string() {
        let list = Json::JsonList(vec![Json::JsonUint(1), Json::JsonBool(true), Json::JsonNull]);
        assert_eq!(list.to_string(), "[1, true, null]");
    }

    #[test]
    fn list_with_strings() {
        let list = Json::JsonList(vec![
            Json::JsonString("a".to_string()),
            Json::JsonString("b".to_string()),
        ]);
        assert_eq!(list.to_string(), r#"["a", "b"]"#);
    }

    #[test]
    fn empty_object_to_string() {
        assert_eq!(Json::JsonObject(vec![]).to_string(), "{}");
    }

    #[test]
    fn object_to_string() {
        let obj = Json::JsonObject(vec![
            ("name".to_string(), Json::JsonString("alice".to_string())),
            ("age".to_string(), Json::JsonUint(30)),
        ]);
        assert_eq!(obj.to_string(), r#"{"name": "alice", "age": 30}"#);
    }

    #[test]
    fn nested_object_in_list() {
        let inner = Json::JsonObject(vec![("x".to_string(), Json::JsonInt(-1))]);
        let list = Json::JsonList(vec![inner, Json::JsonNull]);
        assert_eq!(list.to_string(), r#"[{"x": -1}, null]"#);
    }

    #[test]
    fn display_matches_to_string() {
        let val = Json::JsonUint(99);
        assert_eq!(format!("{}", val), val.to_string());
    }

    #[test]
    fn str_into_json() {
        assert_eq!("hi".to_json().to_string(), "\"hi\"");
    }

    #[test]
    fn string_into_json() {
        assert_eq!("world".to_string().to_json().to_string(), "\"world\"");
    }

    #[test]
    fn unit_into_json() {
        assert_eq!(().to_json().to_string(), "null");
    }

    #[test]
    fn json_into_json() {
        let val = Json::JsonBool(true);
        assert_eq!(val.to_json().to_string(), "true");
    }

    #[test]
    fn into_json_boxed() {
        let boxed: Box<dyn IntoJsonObject> = Box::new("test");
        assert_eq!(boxed.to_json_boxed().to_string(), "\"test\"");
    }
}
