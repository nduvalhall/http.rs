mod http_error;
mod middleware;
mod request;
mod response;
mod route;

use crate::{
    http_error::HttpError, middleware::Middleware, request::Request, response::Response,
    route::Route,
};

struct Context();

fn auth(_: &mut Context, req: Request) -> Result<Request, HttpError> {
    Ok(req)
}

fn index(_: &mut Context, _: Request) -> Result<Response, HttpError> {
    Ok(Response::new())
}

fn main() {
    let _ = Middleware::<Context>::new("GET", "/", auth);
    let _ = Route::<Context>::new("GET", "/", index);
}
