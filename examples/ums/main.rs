use http::{Route, Server};

mod context;
mod database;
mod error;
mod service;
mod user;

fn main() {
    println!("Welcome to the User Management System!");

    let mut server = Server::new("localhost:8090", context::Context::new());

    server.add_route(Route::get("/users", service::print_users));
    server.add_route(Route::post("/users", service::create_user));
    server.add_route(Route::put("/raise-error", service::raise_error));

    server.run();
}
