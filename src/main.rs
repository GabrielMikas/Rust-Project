use axum::routing::Router;
use rocket_testing::*;

#[tokio::main]
async fn main(){
    let app = Router::new().merge(hobbies::routes::routes());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}