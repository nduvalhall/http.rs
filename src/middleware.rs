use crate::{HttpError, Request};

type Handler<C> = fn(&mut C, Request) -> Result<Request, HttpError>;

pub struct Middleware<C> {
    pub path: String,
    handler: Handler<C>,
}

impl<C> Middleware<C> {
    pub fn new(path: &str, handler: Handler<C>) -> Self {
        Self {
            path: path.into(),
            handler,
        }
    }

    pub fn call(&self, ctx: &mut C, req: Request) -> Result<Request, HttpError> {
        (self.handler)(ctx, req)
    }
}
