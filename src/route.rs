use crate::{FromRequest, IntoResponse, Method, Request, Response};

pub struct Route<C> {
    method: Method,
    path: &'static str,
    handler: Box<dyn Fn(&mut C, Request) -> Response>,
}

impl<C: 'static> Route<C> {
    fn wrap<Req, Res>(f: fn(&mut C, Req) -> Res) -> Box<dyn Fn(&mut C, Request) -> Response>
    where
        Req: FromRequest + 'static,
        Res: IntoResponse + 'static,
    {
        Box::new(move |context, request| match Req::from_request(request) {
            Ok(req) => f(context, req).to_response(),
            Err(error) => error.to_response(),
        })
    }

    fn new<Req, Res>(method: Method, path: &'static str, handler: fn(&mut C, Req) -> Res) -> Self
    where
        Req: FromRequest + 'static,
        Res: IntoResponse + 'static,
    {
        Route {
            method,
            path,
            handler: Self::wrap(handler),
        }
    }

    pub fn get_method(&self) -> &Method {
        &self.method
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_handler(&self) -> &Box<dyn Fn(&mut C, Request) -> Response> {
        &self.handler
    }

    pub fn get<Req, Res>(path: &'static str, handler: fn(&mut C, Req) -> Res) -> Self
    where
        Req: FromRequest + 'static,
        Res: IntoResponse + 'static,
    {
        Route::new(Method::Get, path, handler)
    }

    pub fn post<Req, Res>(path: &'static str, handler: fn(&mut C, Req) -> Res) -> Self
    where
        Req: FromRequest + 'static,
        Res: IntoResponse + 'static,
    {
        Route::new(Method::Post, path, handler)
    }

    pub fn put<Req, Res>(path: &'static str, handler: fn(&mut C, Req) -> Res) -> Self
    where
        Req: FromRequest + 'static,
        Res: IntoResponse + 'static,
    {
        Route::new(Method::Put, path, handler)
    }
}
