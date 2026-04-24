use http::{response::Response, route::Route, server::Server};

fn main() {
    let mut server = Server::new("localhost:8080", ());

    server.add_route(Route::new("GET", "/", |_, _| {
        Response::new(http::status_code::StatusCode::NoContent)
    }));

    server.run();
}
