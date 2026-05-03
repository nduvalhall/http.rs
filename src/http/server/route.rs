use crate::http::{
    error::{Error, IntoError},
    request::Request,
    response::{IntoResponse, Response},
};

type Fp<C, R, E> = fn(&mut C, Request) -> Result<R, E>;
type Handler<C> = Box<dyn Fn(&mut C, Request) -> Result<Response, Error>>;

pub struct Route<C> {
    pub method: String,
    pub path: String,
    handler: Handler<C>,
}

impl<C: 'static> Route<C> {
    fn wrap<R, E>(f: Fp<C, R, E>) -> Handler<C>
    where
        R: IntoResponse + 'static,
        E: IntoError + 'static,
    {
        Box::new(move |context, request| match f(context, request) {
            Ok(r) => {
                let r = match r.into_response() {
                    Ok(r) => r,
                    Err(e) => return Err(e.into_error()),
                };

                Ok(r)
            }
            Err(e) => Err(e.into_error()),
        })
    }

    pub fn new<R, E>(method: impl Into<String>, path: impl Into<String>, f: Fp<C, R, E>) -> Self
    where
        R: IntoResponse + 'static,
        E: IntoError + 'static,
    {
        Self {
            method: method.into(),
            path: path.into(),
            handler: Self::wrap(f),
        }
    }

    pub fn call(&self, ctx: &mut C, req: Request) -> Result<Response, Error> {
        (self.handler)(ctx, req)
    }
}
