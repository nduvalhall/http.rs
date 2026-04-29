use crate::{
    raw_request::{FromRawRequest, RawRequest},
    raw_response::{IntoRawResponse, RawResponse},
};

pub struct Route {
    f: Box<dyn Fn(RawRequest) -> RawResponse>,
}

impl Route {
    pub fn new<Req, Res>(f: fn(Req) -> Res) -> Box<dyn Fn(RawRequest) -> RawResponse>
    where
        Req: FromRawRequest + 'static,
        Res: IntoRawResponse + 'static,
    {
        Box::new(
            move |raw_request| match Req::from_raw_request(raw_request) {
                Ok(request) => f(request).into_raw_response(),
                Err(error) => error.into_raw_response(),
            },
        )
    }
}
