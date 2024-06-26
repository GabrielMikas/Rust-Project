use axum::{response::{IntoResponse, Response}, Json};
use serde_json::json;

use crate::utils::errors::Error;

pub async fn response_mapper(res: Response) -> Response{
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.error_mapper());
    let error_response = client_status_error.as_ref()
    .map(|(status_code, error)|{
        let client_error_body = json!({
            "details":{
                "type": error.as_ref()
            }
        });
        (*status_code, Json(client_error_body)).into_response()
    });
     error_response.unwrap_or(res)
}