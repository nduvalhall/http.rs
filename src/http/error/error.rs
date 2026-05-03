pub struct Error();

pub trait IntoError {
    fn into_error(self) -> Error;
}

impl<E: IntoError> From<E> for Error {
    fn from(value: E) -> Self {
        value.into_error()
    }
}
