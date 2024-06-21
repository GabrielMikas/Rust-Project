use crate::models::NewCard;
use axum::{ extract::Path, response::IntoResponse, routing::{delete, get, post}, Json, Router};
use serde_json::{json, Value};
use crate::{create_card, delete_card, list_card_by_id, connect};
use crate::utils::errors::Result;


pub fn routes() -> Router {
    Router::new()
    .route("/yg/:target_id", get(get_by_id))
    .route("/yg/delete/:target_id", delete(delete_handler))
    .route("/yg/create", post(post_handler))
}
    
    
    async fn delete_handler(Path(target_id): Path<i32>) -> impl IntoResponse {
        let mut conn = connect();
        delete_card(&mut conn, &target_id);
        Json( "Deleted ")
    }
    async fn get_by_id(Path(target_id): Path<i32>) -> impl IntoResponse {
        let mut conn = connect();
        list_card_by_id(&mut conn, &target_id)
    }
    async fn post_handler(body: Json<NewCard>) -> Result<Json<Value>>{
      
        let mut conn = connect();
        create_card(&mut conn, body.card_name.to_string(), body.card_code.to_string(), body.card_rarity.to_string(), body.card_amount.to_string(), body.card_edition.to_string(), body.card_url.to_string());
        let response = Json(json!({
            "details":{
                "status":"Registered"
            }
        }));
        Ok(response)

    }

