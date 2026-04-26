struct Request();
trait FromRequest {
    fn from_request(request: Request) -> Self;
}
impl FromRequest for Request {
    fn from_request(request: Request) -> Self {
        request
    }
}

fn handle1(_: Request) {
    println!("handle1");
}

fn handle2(_: Request) {
    println!("handle2");
}

struct Custom();
impl FromRequest for Custom {
    fn from_request(_: Request) -> Self {
        Custom()
    }
}

fn handle3(_: Custom) {
    println!("handle3");
}

struct Route {
    f: Box<dyn Fn(Request)>,
}

impl Route {
    fn wrap<T: FromRequest + 'static>(handler: fn(T)) -> Box<dyn Fn(Request)> {
        Box::new(move |request| handler(T::from_request(request)))
    }

    fn new<T: FromRequest + 'static>(handler: fn(T)) -> Self {
        Route {
            f: Self::wrap(handler),
        }
    }
}

fn main() {
    let mut routes = Vec::new();

    routes.push(Route::new(handle1));
    routes.push(Route::new(handle2));
    routes.push(Route::new(handle3));

    for route in routes {
        (route.f)(Request())
    }
}
