use crate::models::NewCard;
use crate::utils::errors::{Error, Result};
use crate::{connect, create_yg_card, delete_yg_card, list_yg_card_by_id, list_yg_cards};
use axum::{
    extract::Path,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use serde_json::{json, Value};

pub fn routes() -> Router {
    Router::new()
        .route("/yg/:target_id", get(get_by_id_handler_yg).delete(delete_handler_yg))
        .route("/yg", post(post_handler_yg).get(get_all_handler_yg))
}

async fn delete_handler_yg(Path(target_id): Path<i32>) -> impl IntoResponse {
    let mut conn = connect();
    let result = delete_yg_card(&mut conn, &target_id);
    if result == 0 {
        return Error::NotFound
    }
    Error::Accepted    
}
async fn get_by_id_handler_yg(Path(target_id): Path<i32>) -> impl IntoResponse {
    let mut conn = connect();

    #[allow(unreachable_patterns)]
    match list_yg_card_by_id(&mut conn, &target_id) {
        Ok(_) => Ok(()),
        Err(_) => Err(Error::UnmappedError),
        Err(diesel::result::Error::NotFound) => Err(Error::NotFound)
    }
}
async fn post_handler_yg(body: Json<NewCard>) -> Result<Json<Value>> {
    let mut conn = connect();
    create_yg_card(
        &mut conn,
        body.card_name.to_string(),
        body.card_code.to_string(),
        body.card_rarity.to_string(),
        body.card_amount.to_string(),
        body.card_edition.to_string(),
        body.card_url.to_string(),
    );
    let response = Json(json!({
        "details":{
            "status":"Registered"
        }
    }));
    Ok(response)
}
async fn get_all_handler_yg() -> impl IntoResponse {
let mut conn = connect();
list_yg_cards(&mut conn)
}