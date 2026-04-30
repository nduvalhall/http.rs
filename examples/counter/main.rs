use http::{Html, HttpError, IntoJson, Json, JsonValue, Request, Response, Route, Server};

struct Context {
    counter: i32,
}

fn index(_: &mut Context, _: Request) -> Result<Response, HttpError> {
    Ok(Response::ok(Html(include_str!("counter.html").to_string())))
}

struct Count {
    count: i32,
}

impl IntoJson for Count {
    fn into_json(self) -> JsonValue {
        JsonValue::JsonObject(vec![(
            "count".to_string(),
            JsonValue::JsonInt(self.count.into()),
        )])
    }
}

fn get_count(context: &mut Context, _: Request) -> Result<Response, HttpError> {
    Ok(Response::ok(Json(Count {
        count: context.counter,
    })))
}

fn increment(context: &mut Context, _: Request) -> Result<Response, HttpError> {
    context.counter += 1;
    println!("Counter: {}", context.counter);
    Ok(Response::no_content())
}

fn decrement(context: &mut Context, _: Request) -> Result<Response, HttpError> {
    context.counter -= 1;
    println!("Counter: {}", context.counter);
    Ok(Response::no_content())
}

fn main() {
    let counter = Context { counter: 0 };

    Server::new("0.0.0.0:8080", counter)
        .route(Route::new("GET", "/", index))
        .route(Route::new("GET", "/count", get_count))
        .route(Route::new("POST", "/increment", increment))
        .route(Route::new("POST", "/decrement", decrement))
        .run()
}
