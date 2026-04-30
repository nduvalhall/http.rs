use crate::{HttpError, Request, Response};

type Handler<C> = fn(&mut C, Request) -> Result<Response, HttpError>;

pub struct Route<C> {
    pub method: String,
    pub path: String,
    handler: Handler<C>,
}

impl<C> Route<C> {
    pub fn new(method: &str, path: &str, f: Handler<C>) -> Self {
        Route {
            method: method.into(),
            path: path.into(),
            handler: f,
        }
    }

    pub fn call(&self, ctx: &mut C, req: Request) -> Result<Response, HttpError> {
        (self.handler)(ctx, req)
    }
}
