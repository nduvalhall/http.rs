struct Request();
trait FromRequest: Sized {
    fn from_request(request: Request) -> Self;
}
impl FromRequest for Request {
    fn from_request(request: Request) -> Self {
        request
    }
}

struct Response();
trait IntoResponse {
    fn into_response(self) -> Response;
}
impl IntoResponse for Response {
    fn into_response(self) -> Response {
        self
    }
}

struct Error();
trait IntoError {
    fn into_error(self) -> Error;
}
impl IntoError for Error {
    fn into_error(self) -> Error {
        self
    }
}

type Handler<A, B, E> = fn(A) -> Result<B, E>;
type F = Box<dyn Fn(Request) -> Result<Response, Error>>;

struct Route {
    f: F,
}

impl Route {
    fn wrap<A, B, E>(f: Handler<A, B, E>) -> F
    where
        A: FromRequest + 'static,
        B: IntoResponse + 'static,
        E: IntoError + 'static,
    {
        Box::new(move |request| match f(A::from_request(request)) {
            Ok(r) => Ok(r.into_response()),
            Err(e) => Err(e.into_error()),
        })
    }

    fn new<A, B, E>(f: Handler<A, B, E>) -> Self
    where
        A: FromRequest + 'static,
        B: IntoResponse + 'static,
        E: IntoError + 'static,
    {
        Self { f: Self::wrap(f) }
    }
}

enum Json {
    Null,
}
trait FromJson: Sized {
    fn from_json(json: Json) -> Self;
}
trait IntoJson {
    fn into_json(self) -> Json;
}

struct JsonRequest<T: FromJson> {
    body: T,
}
impl<T: FromJson> FromRequest for JsonRequest<T> {
    fn from_request(_: Request) -> Self {
        JsonRequest {
            body: T::from_json(Json::Null),
        }
    }
}

struct JsonResponse<T: IntoJson> {
    body: T,
}
impl<T: IntoJson> IntoResponse for JsonResponse<T> {
    fn into_response(self) -> Response {
        Response()
    }
}

struct Cat(String);
impl FromJson for Cat {
    fn from_json(_: Json) -> Self {
        Cat("Fig".into())
    }
}
impl IntoJson for Cat {
    fn into_json(self) -> Json {
        Json::Null
    }
}

fn pet(request: JsonRequest<Cat>) -> Result<JsonResponse<Cat>, Error> {
    let cat = request.body;
    Ok(JsonResponse { body: cat })
}

fn main() {
    let _ = Route::new(pet);
}
