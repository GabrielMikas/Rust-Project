use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;

pub type Result<T> = core::result::Result<T, Error>;
#[derive(Clone, strum_macros::AsRefStr)]
pub enum Error{
    UnmappedError,
    NotFound,
    InvalidBody,
    Accepted

}
impl IntoResponse for Error{
    fn into_response(self) -> Response{
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(self);
        response
    }
}
impl Error{
    pub fn error_mapper(&self) -> (StatusCode, Error){
        #[allow(unreachable_patterns)]
        match self {
            Self::UnmappedError => (StatusCode::INTERNAL_SERVER_ERROR, Error::UnmappedError),
            Self::NotFound => (StatusCode::NOT_FOUND, Error::NotFound),
            Self::InvalidBody => (StatusCode::BAD_REQUEST, Error::InvalidBody),
            Self::Accepted => (StatusCode::ACCEPTED, Error::Accepted),
        }
    }
}
