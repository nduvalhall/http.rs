//! Convenience re-export of all public types.
//!
//! Add `use amoeba::prelude::*;` to import everything at once.

pub use crate::{
    http_error::HttpError, middleware::Middleware, request::Request, response::ContentType,
    response::Response, route::Route, server::Server,
};
