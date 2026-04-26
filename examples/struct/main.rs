#[derive(Debug)]
enum StatusCode {
    Ok,
    InternalServerError,
}

struct Request {}
trait FromRequest {
    fn from_request(request: Request) -> Self;
}
impl FromRequest for Request {
    fn from_request(request: Request) -> Self {
        request
    }
}

#[derive(Debug)]
struct Response {
    status_code: StatusCode,
    body: Option<String>,
}
trait ToResponse {
    fn to_response(self) -> Response;
}
impl ToResponse for Response {
    fn to_response(self) -> Response {
        self
    }
}

struct Route<C: 'static> {
    f: Box<dyn Fn(&mut C, Request) -> Response>,
}

impl<C: 'static> Route<C> {
    fn wrap<Req: FromRequest + 'static, Res: ToResponse + 'static, Err: ToResponse + 'static>(
        f: fn(&mut C, Req) -> Result<Res, Err>,
    ) -> Box<dyn Fn(&mut C, Request) -> Response> {
        Box::new(
            move |context, request| match f(context, Req::from_request(request)) {
                Ok(res) => res.to_response(),
                Err(error) => error.to_response(),
            },
        )
    }

    fn new<Req: FromRequest + 'static, Res: ToResponse + 'static, Err: ToResponse + 'static>(
        f: fn(&mut C, Req) -> Result<Res, Err>,
    ) -> Self {
        Route { f: Self::wrap(f) }
    }
}

struct Person {}
impl FromRequest for Person {
    fn from_request(_: Request) -> Self {
        Person {}
    }
}

enum Error {
    UnknownError(String),
}

impl ToResponse for Error {
    fn to_response(self) -> Response {
        match self {
            Self::UnknownError(message) => Response {
                status_code: StatusCode::InternalServerError,
                body: Some(message),
            },
        }
    }
}

fn make_error() -> Result<(), Error> {
    Err(Error::UnknownError(String::from("i made an error")))
}

fn get_person(_: &mut (), _: Person) -> Result<Response, Error> {
    let _ = make_error()?;
    Ok(Response {
        status_code: StatusCode::Ok,
        body: None,
    })
}

fn main() {
    let route = Route::<()>::new(get_person);
    let request = Request {};
    let response = (route.f)(&mut (), request);
    println!("{:?}", response);
}
