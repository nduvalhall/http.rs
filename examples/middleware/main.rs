use amoeba::{HttpError, Middleware, Request, Response, Route, Server};

const API_KEY: &str = "secret";

struct Context();

fn auth(_: &mut Context, request: Request) -> Result<Request, HttpError> {
    match request.headers.get("x-api-key") {
        Some(key) if key == API_KEY => Ok(request),
        _ => Err(HttpError::new(401, "Incorrect API key provided")),
    }
}

fn index(_: &mut Context, _: Request) -> Result<Response, HttpError> {
    Ok(Response::ok("Hello, world!"))
}

fn main() {
    Server::new("localhost:8080", Context())
        .middleware(Middleware::new("*", auth))
        .route(Route::new("GET", "/", index))
        .run();
}
