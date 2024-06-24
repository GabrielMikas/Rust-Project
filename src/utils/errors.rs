use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;

pub type Result<T> = core::result::Result<T, Error>;

pub enum Error{
    UnmappedError,
    NotFound,
    InvalidBody,
    Accepted

}
impl IntoResponse for Error{
    fn into_response(self) -> Response{
        let body = match self {
            Self::UnmappedError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::InvalidBody => StatusCode::BAD_REQUEST,
            Self::Accepted => StatusCode::ACCEPTED
        };
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
