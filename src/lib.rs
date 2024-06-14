use axum::Json;
use diesel::{pg::PgConnection};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env::{self};
use self::models::{Card, NewCard};
pub mod models;
pub mod schema;
pub mod errors;

pub fn connect() -> PgConnection {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("Could not fetch database url");
    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}
pub fn create_card(conn: &mut PgConnection, name: String, code: String, rarity: String, amount: String, edition: String, url: String) -> Card{
    use crate::schema::yg;
    let new_card = NewCard {
        card_name: name,
        card_code: code,
        card_rarity: rarity,
        card_amount: amount,
        card_edition: edition,
        card_url: url
    };

    diesel::insert_into(yg::table)
        .values(&new_card)
        .returning(Card::as_returning())
        .get_result(conn)
        .expect("Could not insert into the database")
}
// pub fn list_cards(conn: &mut PgConnection) -> Json<Vec<Card>>{
//     use self::schema::yg::dsl::*;

//     let results =  yg
//         .load::<Card>(conn)
//         .expect("Error loading cards");
//     Json(results)
// }
pub fn delete_card(conn: &mut PgConnection, target_id: &i32){
    use self::schema::yg::dsl::*;

    diesel::delete
        (yg.filter(id.eq(target_id)))
        .execute(conn)
        .expect("Error deleting card");
}
pub fn list_card_by_id(conn: &mut PgConnection, target_id: &i32) -> Json<Card>{
    use self::schema::yg::dsl::*;
    
    let result = yg
        .filter(id.eq(target_id))
        .first::<Card>(conn)
        .expect("Could not find card by Id");

        Json(result)
    
}