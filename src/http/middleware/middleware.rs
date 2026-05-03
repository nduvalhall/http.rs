use crate::http::{
    error::{Error, IntoError},
    request::{IntoRequest, Request},
};

type Fp<C, E> = fn(&mut C, Request) -> Result<Request, E>;
type Handler<C> = Box<dyn Fn(&mut C, Request) -> Result<Request, Error>>;

pub struct Middleware<C> {
    pub path: String,
    handler: Handler<C>,
}

impl<C: 'static> Middleware<C> {
    fn wrap<E>(f: Fp<C, E>) -> Handler<C>
    where
        E: IntoError + 'static,
    {
        Box::new(move |context, request| match f(context, request) {
            Ok(r) => Ok(r.into_request()),
            Err(e) => Err(e.into_error()),
        })
    }

    pub fn new<E>(path: impl Into<String>, f: Fp<C, E>) -> Self
    where
        E: IntoError + 'static,
    {
        Self {
            path: path.into(),
            handler: Self::wrap(f),
        }
    }

    pub fn call(&self, ctx: &mut C, req: Request) -> Result<Request, Error> {
        (self.handler)(ctx, req)
    }
}
