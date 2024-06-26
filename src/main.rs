use axum::{middleware, routing::Router};
use middlewares::response_mapper::response_mapper;
use rocket_testing::*;

#[tokio::main]
async fn main(){
    let app = 
    Router::new().merge(hobbies::routes::routes()).layer(middleware::map_response(response_mapper));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}