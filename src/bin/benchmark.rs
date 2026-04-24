use http::{Response, Route, Server, StatusCode};

fn main() {
    let mut server = Server::new("localhost:8080", ());

    server.add_route(Route::new("GET", "/", |_, _| {
        Response::new(StatusCode::NoContent)
    }));

    server.run();
}
