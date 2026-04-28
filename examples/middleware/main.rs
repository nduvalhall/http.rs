use http::prelude::*;

const API_KEY: &str = "secret";

struct Context {}

fn auth(_: &mut Context, request: Request) -> Result<Request, Response> {
    match request.get_header("x-api-key") {
        Some(key) if key == API_KEY => Ok(request),
        _ => Err(Response::unauthorized()),
    }
}

fn index(_: &mut Context, _: Request) -> &'static str {
    "Hello, world!"
}

fn main() {
    let mut server = Server::new("localhost:8080", Context {});

    server.add_middleware(Middleware::new("*", auth));
    server.add_route(Route::get("/", index));

    server.run();
}
