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
    fn wrap<Req: FromRequest + 'static, Res: ToResponse + 'static>(
        handler: fn(&mut C, Req) -> Res,
    ) -> Box<dyn Fn(&mut C, Request) -> Response> {
        Box::new(move |context, request| handler(context, Req::from_request(request)).to_response())
    }

    pub fn new<Req: FromRequest + 'static, Res: ToResponse + 'static>(
        method: &'static str,
        path: &'static str,
        handler: fn(&mut C, Req) -> Res,
    ) -> Self {
        Route {
            method,
            path,
            handler: Self::wrap(handler),
        }
    }
}
