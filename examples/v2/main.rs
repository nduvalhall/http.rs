#![allow(unused)]

use crate::{
    json::{FromJson, IntoJson, Json, JsonValue},
    request::Request,
    response::Response,
    route::Route,
};

mod http_error;
mod json;
mod raw_request;
mod raw_response;
mod request;
mod response;
mod route;

struct Cat {
    name: String,
}

impl FromJson for Cat {
    fn from_json(json: JsonValue) -> Self {
        Cat { name: "Fig".into() }
    }
}

impl IntoJson for Cat {
    fn into_json(self) -> JsonValue {
        JsonValue::JsonString(self.name)
    }
}

fn handler(req: Request<Json<Cat>>) -> Response<Json<Cat>> {
    if let Some(Json(cat)) = req.body {
        println!("{}", cat.name);
        Response {
            status_code: 200,
            headers: vec![],
            body: Some(Json(cat)),
        }
    } else {
        Response {
            status_code: 200,
            headers: vec![],
            body: None,
        }
    }
}

fn main() {
    let _ = Route::new(handler);
}
