/// An HTTP error returned from a handler or middleware; the server converts it to a plain-text response.
pub struct HttpError {
    /// HTTP status code sent to the client.
    pub status: u16,
    /// Error message sent as the response body.
    pub message: String,
}

impl HttpError {
    /// Creates an error with status `500` and the given message.
    pub fn new(msg: impl Into<String>) -> Self {
        Self {
            status: 500,
            message: msg.into(),
        }
    }

    /// Overrides the HTTP status code.
    pub fn status(mut self, status: u16) -> Self {
        self.status = status;
        self
    }
}
