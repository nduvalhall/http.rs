use amoeba::{HttpError, Request, Response, Route, Server};

struct Ctx();

fn index(_: &mut Ctx, _: Request) -> Result<Response, HttpError> {
    println!("index endpoint called");
    Ok(Response::no_content())
}

fn main() {
    Server::new("localhost:8080", Ctx())
        .route(Route::new("GET", "/", index))
        .run();
}
