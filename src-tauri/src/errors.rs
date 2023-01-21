use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
pub(crate) enum MyError {
    #[error("sending HTTP request")]
    HttpError,

    #[error("bad response: {0}")]
    BadResponse(u16),
}
pub(crate) type Result<T> = std::result::Result<T, MyError>;
