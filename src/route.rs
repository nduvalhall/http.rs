use crate::{http_error::HttpError, request::Request, response::Response};

/// A single route: an HTTP method, an exact path, and a handler function.
pub struct Route<C> {
    /// HTTP method this route accepts (e.g. `"GET"`, `"POST"`).
    pub method: String,
    /// Exact URL path this route matches.
    pub path: String,
    /// Handler called when the route matches.
    pub handler: fn(&mut C, Request) -> Result<Response, HttpError>,
}

impl<C> Route<C> {
    /// Creates a new route.
    ///
    /// `method` is any HTTP verb string. `path` is matched exactly — no wildcards or
    /// path parameters. The server returns `404` if no path matches and `405` if the
    /// path matches but the method does not.
    pub fn new(
        method: impl Into<String>,
        path: impl Into<String>,
        handler: fn(&mut C, Request) -> Result<Response, HttpError>,
    ) -> Self {
        Self {
            method: method.into(),
            path: path.into(),
            handler,
        }
    }
}
