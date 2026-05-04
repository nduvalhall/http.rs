#![allow(dead_code)]

// --- Request --- //
struct Request();

trait FromRequest {
    fn from_request(req: &Request) -> Self
    where
        Self: Sized;
}

impl FromRequest for Request {
    fn from_request(_: &Request) -> Self {
        Self()
    }
}

// --- Response --- //
struct Response();

trait IntoResponse {
    fn into_response(self) -> Response;
}

impl IntoResponse for Response {
    fn into_response(self) -> Response {
        self
    }
}

fn endpoint(req: Request) -> Response {}

// --- IntoEndpoint --- //
trait IntoEndpoint<Args> {
    fn into_endpoint(self) -> Box<dyn Fn(Request) -> Response>;
}

impl<A, F> IntoEndpoint<(A,)> for F
where
    A: FromRequest + 'static,
    F: Fn(A) -> Response + 'static,
{
    fn into_endpoint(self) -> Box<dyn Fn(Request) -> Response> {
        Box::new(move |req| self(A::from_request(&req)))
    }
}

impl<A, B, F> IntoEndpoint<(A, B)> for F
where
    A: FromRequest + 'static,
    B: FromRequest + 'static,
    F: Fn(A, B) -> Response + 'static,
{
    fn into_endpoint(self) -> Box<dyn Fn(Request) -> Response> {
        Box::new(move |req| self(A::from_request(&req), B::from_request(&req)))
    }
}

impl<A, B, C, F> IntoEndpoint<(A, B, C)> for F
where
    A: FromRequest + 'static,
    B: FromRequest + 'static,
    C: FromRequest + 'static,
    F: Fn(A, B, C) -> Response + 'static,
{
    fn into_endpoint(self) -> Box<dyn Fn(Request) -> Response> {
        Box::new(move |req| {
            self(
                A::from_request(&req),
                B::from_request(&req),
                C::from_request(&req),
            )
        })
    }
}

// --- Route --- //
struct Route {
    f: Box<dyn Fn(Request) -> Response>,
}

impl Route {
    fn new<Args, F: IntoEndpoint<Args> + 'static>(f: F) -> Self {
        Self {
            f: f.into_endpoint(),
        }
    }

    fn call(&self, req: Request) -> Response {
        (self.f)(req)
    }
}

// --- Extractors --- //
struct Path<T: From<String>>(T);

impl<T: From<String>> FromRequest for Path<T> {
    fn from_request(_: &Request) -> Self
    where
        Self: Sized,
    {
        Path(T::from("Hello".into()))
    }
}

struct Query<T: From<String>>(T);

impl<T: From<String>> FromRequest for Query<T> {
    fn from_request(_: &Request) -> Self
    where
        Self: Sized,
    {
        Query(T::from("Hello".into()))
    }
}

// --- Application --- //
struct John(String);

impl John {
    fn new(name: &str) -> Self {
        Self(name.into())
    }
}

impl FromRequest for John {
    fn from_request(_: &Request) -> Self
    where
        Self: Sized,
    {
        John::new("John")
    }
}

struct Alice(String);

impl Alice {
    fn new(name: &str) -> Self {
        Self(name.into())
    }
}

impl FromRequest for Alice {
    fn from_request(_: &Request) -> Self
    where
        Self: Sized,
    {
        Alice::new("Alice")
    }
}

fn endpoint1(_: Request) -> Response {
    Response()
}

fn endpoint2(j: John, a: Alice) -> Response {
    println!("{} and {}", j.0, a.0);
    Response()
}

fn main() {
    let a = Route::new(endpoint1);
    let b = Route::new(endpoint2);

    let req = Request();
    let _ = a.call(req);

    let req = Request();
    let _ = b.call(req);
}
