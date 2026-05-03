use amoeba::{Error, HtmlResponse, IntoJson, Json, JsonResponse, Request, Response, Route, Server};

struct Context {
    counter: i32,
}

fn index(_: &mut Context, _: Request) -> Result<HtmlResponse, Error> {
    match HtmlResponse::from_file("examples/counter/index.html") {
        Ok(r) => Ok(r),
        Err(()) => Err(Error::new(500, "file not found")),
    }
}

struct Count {
    count: i32,
}

impl IntoJson for Count {
    fn into_json(&self) -> Json {
        Json::object(vec![("count", Json::number(self.count))])
    }
}

fn get_count(context: &mut Context, _: Request) -> Result<JsonResponse<Count>, Error> {
    Ok(JsonResponse::new(Count {
        count: context.counter,
    }))
}

fn increment(context: &mut Context, _: Request) -> Result<Response, Error> {
    context.counter += 1;
    println!("Counter: {}", context.counter);
    Ok(Response::new().status_code(204))
}

fn decrement(context: &mut Context, _: Request) -> Result<Response, Error> {
    context.counter -= 1;
    println!("Counter: {}", context.counter);
    Ok(Response::new().status_code(204))
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
