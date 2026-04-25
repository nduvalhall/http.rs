struct Request {}

trait FromRequest {
    fn from_request(request: Request) -> Self;
}

impl FromRequest for Request {
    fn from_request(request: Request) -> Self {
        request
    }
}

struct Route<F: FromRequest> {
    f: fn(F),
}

struct Server<F: FromRequest> {
    routes: Vec<Route<F>>,
}

impl<F: FromRequest> Server<F> {
    fn run(&self) {
        for route in &self.routes {
            let request = Request {};
            (route.f)(F::from_request(request))
        }
    }
}

fn route1(_: Request) {
    println!("Route 1");
}

fn route2(_: Request) {
    println!("Route 2")
}

fn main() {
    let mut server = Server { routes: Vec::new() };

    server.routes.push(Route { f: route1 });
    server.routes.push(Route { f: route2 });
    // server.routes.push(Route { f: create_person });

    server.run();
}
