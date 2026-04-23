use std::collections::HashMap;

enum StatusCode {
    Ok,
    NotFound,
}

impl StatusCode {
    fn to_str(self) -> &'static str {
        match self {
            Self::Ok => "200 OK",
            Self::NotFound => "404 Not Found",
        }
    }
}

struct Request {}

impl Request {
    fn new() -> Self {
        Request {}
    }
}

struct Response {
    status_code: StatusCode,
}

impl Response {
    fn new(status_code: StatusCode) -> Self {
        Response { status_code }
    }
}

type Handler<C> = fn(&mut C, Request) -> Response;

struct Route<C> {
    path: &'static str,
    handler: Handler<C>,
}

impl<C> Route<C> {
    fn new(path: &'static str, handler: Handler<C>) -> Self {
        Self { path, handler }
    }
}

struct Server<C> {
    address: &'static str,
    context: C,
    routes: HashMap<&'static str, Route<C>>,
}

impl<C> Server<C> {
    fn new(address: &'static str, context: C) -> Self {
        Self {
            address,
            context,
            routes: HashMap::new(),
        }
    }

    fn add_route(&mut self, route: Route<C>) {
        self.routes.insert(route.path, route);
    }

    fn dispatch(&mut self, path: &'static str, request: Request) -> Response {
        match self.routes.get(path) {
            Some(route) => (route.handler)(&mut self.context, request),
            None => Response::new(StatusCode::NotFound),
        }
    }

    fn run(mut self) {
        // implement TCP server here
        println!("Listening on {}", self.address);

        let response = self.dispatch("GET /increment", Request::new());
        println!("{}", response.status_code.to_str());

        let response = self.dispatch("GET /decrement", Request::new());
        println!("{}", response.status_code.to_str());

        let response = self.dispatch("GET /not-a-path", Request::new());
        println!("{}", response.status_code.to_str());
    }
}

struct Counter(i32);

fn increment(counter: &mut Counter, _request: Request) -> Response {
    counter.0 += 1;
    println!("Counter: {}", counter.0);
    Response::new(StatusCode::Ok)
}

fn decrement(counter: &mut Counter, _request: Request) -> Response {
    counter.0 -= 1;
    println!("Counter: {}", counter.0);
    Response::new(StatusCode::Ok)
}

fn main() {
    let counter = Counter(0);

    let mut server = Server::new("127.0.0.1:42069", counter);

    server.add_route(Route::new("GET /increment", increment));
    server.add_route(Route::new("GET /decrement", decrement));

    server.run();
}
