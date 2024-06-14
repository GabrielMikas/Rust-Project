use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;

pub type Result<T> = core::result::Result<T, Error>;

pub enum Error{
    FailedToRegister,
}
impl IntoResponse for Error{
    fn into_response(self) -> Response{
        (StatusCode::INTERNAL_SERVER_ERROR).into_response()
    }
}