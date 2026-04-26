use std::fmt::Debug;

use http::{FromRequest, Pipe, Request};

pub struct User {
    name: String,
    age: u8,
}

impl FromRequest for User {
    /// Request body is in format name|age
    fn from_request(request: Request) -> Self {
        let pipe = Pipe::from_string(&request.body);
        User {
            name: pipe.get("name").unwrap().trim().to_string(),
            age: u8::from_str_radix(pipe.get("age").unwrap().trim(), 10).unwrap(),
        }
    }
}

impl Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("User(name: {}, age: {})", self.name, self.age);
        f.write_str(&s)
    }
}
