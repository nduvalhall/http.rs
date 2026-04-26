use http::{Route, Server};

mod context;
mod database;
mod service;
mod user;

fn main() {
    println!("Welcome to the User Management System!");

    let context = context::Context::new();

    let mut server = Server::new("localhost:8090", context);

    server.add_route(Route::new("GET", "/users", service::print_users));
    server.add_route(Route::new("POST", "/users", service::create_user));

    server.run();
}
