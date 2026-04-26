use http::{Request, Response, Route, Server, StatusCode};

fn index(_: &mut (), _: Request) -> Response {
    println!("index endpoint called");
    Response::new(StatusCode::NoContent)
}

fn main() {
    let mut server = Server::new("localhost:8080", ());

    server.add_route(Route::new("GET", "/", index));

    server.run();
}
