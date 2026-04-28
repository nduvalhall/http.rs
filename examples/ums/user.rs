use std::fmt::Debug;

use http::prelude::*;

pub struct User {
    name: String,
    age: u8,
}

impl FromRequest for User {
    fn from_request(request: Request) -> Result<Self, Response> {
        let pipe =
            Pipe::from_str(&request.body).or_internal_server_error("Failed to parse pipe")?;

        let name = pipe
            .get("name")
            .or_internal_server_error("Field 'name' not found in pipe")?;

        let age = pipe
            .get("age")
            .and_then(|v| u8::from_str_radix(&v, 10).ok())
            .or_internal_server_error("Field 'age' not found in pipe")?;

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
        let pipe =
            Pipe::from_str(&request.body).or_internal_server_error("Failed to parse pipe")?;

        let error = pipe
            .get("error")
            .or_internal_server_error("Field 'error' not found in pipe")?;

        Err(Response::internal_server_error(error))
    }
}
