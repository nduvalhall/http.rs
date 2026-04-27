use crate::{
    Method, Response,
    request::{FromRequest, Request},
    response::IntoResponse,
};

pub struct Route<C> {
    pub method: Method,
    pub path: &'static str,
    pub handler: Box<dyn Fn(&mut C, Request) -> Response>,
}

impl<C: 'static> Route<C> {
    pub fn wrap<Req, Res, Err>(
        f: fn(&mut C, Req) -> Result<Res, Err>,
    ) -> Box<dyn Fn(&mut C, Request) -> Response>
    where
        Req: FromRequest + 'static,
        Res: IntoResponse + 'static,
        Err: IntoResponse + 'static,
    {
        Box::new(move |context, request| match Req::from_request(request) {
            Ok(req) => match f(context, req) {
                Ok(req) => req.to_response(),
                Err(error) => error.to_response(),
            },
            Err(error) => error.to_response(),
        })
    }

    fn new<Req, Res, Err>(
        method: Method,
        path: &'static str,
        handler: fn(&mut C, Req) -> Result<Res, Err>,
    ) -> Self
    where
        Req: FromRequest + 'static,
        Res: IntoResponse + 'static,
        Err: IntoResponse + 'static,
    {
        Route {
            method,
            path,
            handler: Self::wrap(handler),
        }
    }

    pub fn get<Req, Res, Err>(
        path: &'static str,
        handler: fn(&mut C, Req) -> Result<Res, Err>,
    ) -> Self
    where
        Req: FromRequest + 'static,
        Res: IntoResponse + 'static,
        Err: IntoResponse + 'static,
    {
        Route::new(Method::Get, path, handler)
    }

    pub fn post<Req, Res, Err>(
        path: &'static str,
        handler: fn(&mut C, Req) -> Result<Res, Err>,
    ) -> Self
    where
        Req: FromRequest + 'static,
        Res: IntoResponse + 'static,
        Err: IntoResponse + 'static,
    {
        Route::new(Method::Post, path, handler)
    }

    pub fn put<Req, Res, Err>(
        path: &'static str,
        handler: fn(&mut C, Req) -> Result<Res, Err>,
    ) -> Self
    where
        Req: FromRequest + 'static,
        Res: IntoResponse + 'static,
        Err: IntoResponse + 'static,
    {
        Route::new(Method::Put, path, handler)
    }
}
