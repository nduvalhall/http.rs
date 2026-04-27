use crate::{Request, Response};

pub struct Middleware<C> {
    path: &'static str,
    handler: Box<dyn Fn(&mut C, Request) -> Result<Request, Response>>,
}

impl<C: 'static> Middleware<C> {
    fn wrap(
        f: fn(&mut C, Request) -> Result<Request, Response>,
    ) -> Box<dyn Fn(&mut C, Request) -> Result<Request, Response>> {
        Box::new(move |context, request| f(context, request))
    }

    pub fn new(
        path: &'static str,
        handler: fn(&mut C, Request) -> Result<Request, Response>,
    ) -> Self {
        Middleware {
            path,
            handler: Self::wrap(handler),
        }
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_handler(&self) -> &Box<dyn Fn(&mut C, Request) -> Result<Request, Response>> {
        &self.handler
    }
}
