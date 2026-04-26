use crate::{
    Response,
    request::{FromRequest, Request},
    response::ToResponse,
};

pub struct Route<C> {
    pub method: &'static str,
    pub path: &'static str,
    pub handler: Box<dyn Fn(&mut C, Request) -> Response>,
}

impl<C: 'static> Route<C> {
    pub fn wrap<
        Req: FromRequest + 'static,
        Res: ToResponse + 'static,
        Err: ToResponse + 'static,
    >(
        f: fn(&mut C, Req) -> Result<Res, Err>,
    ) -> Box<dyn Fn(&mut C, Request) -> Response> {
        Box::new(
            move |context, request| match f(context, Req::from_request(request)) {
                Ok(res) => res.to_response(),
                Err(error) => error.to_response(),
            },
        )
    }

    pub fn new<Req: FromRequest + 'static, Res: ToResponse + 'static, Err: ToResponse + 'static>(
        method: &'static str,
        path: &'static str,
        handler: fn(&mut C, Req) -> Result<Res, Err>,
    ) -> Self {
        Route {
            method,
            path,
            handler: Self::wrap(handler),
        }
    }
}
