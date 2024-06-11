
use models::{Card, NewCard};
use rocket_testing::*;
use axum::{ extract::Path, response::IntoResponse, routing::{delete, get}, Json, Router};

#[tokio::main]
async fn main(){
    let app = Router::new()
    .route("/yg/:target_id", get(get_by_id))
    .route("/yg/delete/:target_id", delete(delete_handler));

    
    
    async fn delete_handler(Path(target_id): Path<i32>) -> impl IntoResponse {
        let mut conn = connect();
        delete_card(&mut conn, &target_id);
        Json( "Deleted ")
    }
    async fn get_by_id(Path(target_id): Path<i32>) -> impl IntoResponse {
        let mut conn = connect();
        list_card_by_id(&mut conn, &target_id)
    }

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}