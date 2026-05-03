use crate::http::{
    error::{Error, IntoError},
    request::{FromRequest, IntoRequest, Request},
};

type Fp<C, A, B, E> = fn(&mut C, A) -> Result<B, E>;
type Handler<C> = Box<dyn Fn(&mut C, Request) -> Result<Request, Error>>;

pub struct Middleware<C> {
    pub path: String,
    handler: Handler<C>,
}

impl<C: 'static> Middleware<C> {
    fn wrap<A, B, E>(f: Fp<C, A, B, E>) -> Handler<C>
    where
        A: FromRequest + 'static,
        B: IntoRequest + 'static,
        E: IntoError + 'static,
    {
        Box::new(move |context, request| {
            let a = A::from_request(request)?;
            match f(context, a) {
                Ok(r) => Ok(r.into_request()),
                Err(e) => Err(e.into_error()),
            }
        })
    }

    pub fn new<A, B, E>(path: impl Into<String>, f: Fp<C, A, B, E>) -> Self
    where
        A: FromRequest + 'static,
        B: IntoRequest + 'static,
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
