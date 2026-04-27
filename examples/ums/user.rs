use std::fmt::Debug;

use http::{FromRequest, Pipe, Request};

use crate::error::UMSError;

pub struct User {
    name: String,
    age: u8,
}

impl FromRequest for User {
    type Error = ();
    fn from_request(request: Request) -> Result<Self, Self::Error> {
        let pipe = Pipe::from_string(&request.body);
        Ok(User {
            name: pipe.get("name").unwrap().trim().to_owned(),
            age: u8::from_str_radix(pipe.get("age").unwrap().trim(), 10).unwrap(),
        })
    }
}

impl Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("User(name: {}, age: {})", self.name, self.age);
        f.write_str(&s)
    }
}

pub struct RaiseError {
    pub error: String,
}

impl FromRequest for RaiseError {
    type Error = UMSError;
    fn from_request(request: Request) -> Result<Self, Self::Error> {
        let pipe = Pipe::from_string(&request.body);
        Err(UMSError::UnknownError(
            pipe.get("error").unwrap().to_owned(),
        ))
    }
}
