use std::fmt::Debug;

use http::{FromRequest, Request, Route, Server};

struct User {
    name: String,
    age: u8,
}

impl FromRequest for User {
    /// Request body is in format name|age
    fn from_request(request: Request) -> Self {
        let (name, age) = request.body.split_once('|').unwrap();
        User {
            name: name.trim().to_string(),
            age: u8::from_str_radix(age.trim(), 10).unwrap(),
        }
    }
}

impl Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("User(name: {}, age: {})", self.name, self.age);
        f.write_str(&s)
    }
}

struct Context {
    users: Vec<User>,
}

fn create_user(context: &mut Context, user: User) {
    context.users.push(user);
}

fn print_users(context: &mut Context, _: Request) {
    println!("Users: {:?}", context.users);
}

fn main() {
    let context = Context { users: Vec::new() };
    let mut server = Server::new("localhost:8087", context);

    server.add_route(Route::new("GET", "/users", print_users));
    server.add_route(Route::new("POST", "/users", create_user));

    server.run();
}
