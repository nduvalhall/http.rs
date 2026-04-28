use http::prelude::*;

struct Context {
    counter: i32,
}

fn index(_: &mut Context, _: Request) -> Response {
    Response::ok(&include_str!("counter.html").to_string())
}

struct Count {
    count: i32,
}

impl IntoJson for Count {
    fn to_json(self) -> Json {
        Json::JsonObject(vec![(
            "count".to_string(),
            Json::JsonInt(self.count.into()),
        )])
    }
}

fn get_count(context: &mut Context, _: Request) -> Response {
    Response::ok(
        Count {
            count: context.counter,
        }
        .to_json()
        .to_string()
        .as_str(),
    )
}

fn increment(context: &mut Context, _: Request) {
    context.counter += 1;
    println!("Counter: {}", context.counter);
}

fn decrement(context: &mut Context, _: Request) {
    context.counter -= 1;
    println!("Counter: {}", context.counter);
}

fn main() {
    let counter = Context { counter: 0 };

    let mut server = Server::new("0.0.0.0:42069", counter);

    server.add_route(Route::get("/", index));
    server.add_route(Route::get("/count", get_count));
    server.add_route(Route::post("/increment", increment));
    server.add_route(Route::post("/decrement", decrement));

    server.run();
}
