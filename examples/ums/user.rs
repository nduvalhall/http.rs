use std::fmt::Debug;

use http::{FromRequest, Pipe, Request, Response};

pub struct User {
    name: String,
    age: u8,
}

impl FromRequest for User {
    fn from_request(request: Request) -> Result<Self, Response> {
        let Some(pipe) = Pipe::from_str(&request.body) else {
            return Err(Response::InternalServerError(
                "Failed to parse pipe".to_string(),
            ));
        };

        let Some(name) = pipe.get("name") else {
            return Err(Response::InternalServerError(
                "Field 'name' not found in pipe".to_string(),
            ));
        };

        let Some(age) = pipe
            .get("age")
            .and_then(|v| u8::from_str_radix(&v, 10).ok())
        else {
            return Err(Response::InternalServerError(
                "Field 'age' not found in pipe".to_string(),
            ));
        };

        Ok(User { name, age })
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
    fn from_request(request: Request) -> Result<Self, Response> {
        let Some(pipe) = Pipe::from_str(&request.body) else {
            return Err(Response::InternalServerError(
                "Failed to parse pipe".to_string(),
            ));
        };

        let Some(error) = pipe.get("error") else {
            return Err(Response::InternalServerError(
                "Field 'error' not found in pipe".to_string(),
            ));
        };

        Err(Response::InternalServerError(error))
    }
}
