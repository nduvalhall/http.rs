use crate::{request::Request, response::Response};

type Handler<C> = fn(&mut C, Request) -> Response;

pub struct Route<C> {
    pub method: &'static str,
    pub path: &'static str,
    pub handler: Handler<C>,
}

impl<C> Route<C> {
    pub fn new(method: &'static str, path: &'static str, handler: Handler<C>) -> Self {
        Self {
            method,
            path,
            handler,
        }
    }
}
