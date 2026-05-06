use crate::{http_error::HttpError, request::Request};

/// A middleware that intercepts a request before it reaches the route handler.
///
/// Returns the (possibly modified) [`Request`] to continue, or an [`HttpError`] to short-circuit.
pub struct Middleware<C> {
    /// HTTP method filter; use `"*"` to match any method.
    pub method: String,
    /// Path filter; use `"*"` to match any path.
    pub path: String,
    /// Handler called when method and path both match.
    pub handler: fn(&mut C, Request) -> Result<Request, HttpError>,
}

impl<C> Middleware<C> {
    /// Creates a new middleware.
    ///
    /// Use `"*"` for `method` or `path` to match any value.
    /// Middleware runs in registration order before the matching route handler.
    pub fn new(
        method: impl Into<String>,
        path: impl Into<String>,
        f: fn(&mut C, Request) -> Result<Request, HttpError>,
    ) -> Self {
        Self {
            method: method.into(),
            path: path.into(),
            handler: f,
        }
    }
}
