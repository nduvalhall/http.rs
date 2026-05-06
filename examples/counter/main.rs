use amoeba::prelude::*;

struct Counter(i32);

fn increment(counter: &mut Counter, _: Request) -> Result<Response, HttpError> {
    counter.0 += 1;
    Ok(Response::new())
}

fn decrement(counter: &mut Counter, _: Request) -> Result<Response, HttpError> {
    counter.0 -= 1;
    Ok(Response::new())
}

fn get_count(counter: &mut Counter, _: Request) -> Result<Response, HttpError> {
    let count = counter.0.to_string().into_bytes();
    Ok(Response::new()
        .body(ContentType::PlainText, count)
        .status(200))
}

fn main() {
    Server::new("localhost:8080", Counter(0))
        .route(Route::new("GET", "/count", get_count))
        .route(Route::new("POST", "/increment", increment))
        .route(Route::new("POST", "/decrement", decrement))
        .run()
}
