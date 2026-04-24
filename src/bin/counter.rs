use http::{
    request::Request, response::Response, route::Route, server::Server, status_code::StatusCode,
};

struct Counter(i32);

fn index(_: &mut Counter, _: Request) -> Response {
    Response::with_body(StatusCode::Ok, Vec::from(include_str!("counter.html")))
}

fn count(counter: &mut Counter, _: Request) -> Response {
    let response = format!(r#"{{"count": {}}}"#, counter.0);
    Response::with_body(StatusCode::Ok, Vec::from(response.as_bytes()))
}

fn increment(counter: &mut Counter, _: Request) -> Response {
    counter.0 += 1;
    println!("Counter: {}", counter.0);
    Response::new(StatusCode::NoContent)
}

fn decrement(counter: &mut Counter, _: Request) -> Response {
    counter.0 -= 1;
    println!("Counter: {}", counter.0);
    Response::new(StatusCode::NoContent)
}

fn main() {
    let counter = Counter(0);

    let mut server = Server::new("127.0.0.1:42069", counter);

    server.add_route(Route::new("GET", "/", index));
    server.add_route(Route::new("GET", "/count", count));
    server.add_route(Route::new("POST", "/increment", increment));
    server.add_route(Route::new("POST", "/decrement", decrement));

    server.run();
}
