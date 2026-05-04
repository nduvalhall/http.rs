pub struct HttpError {
    pub status: u16,
    pub message: String,
}

impl HttpError {
    pub fn new(msg: impl Into<String>) -> Self {
        Self {
            status: 500,
            message: msg.into(),
        }
    }

    pub fn status(mut self, status: u16) -> Self {
        self.status = status;
        self
    }
}
