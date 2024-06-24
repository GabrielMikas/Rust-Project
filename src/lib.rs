use axum::Json;
use diesel::{prelude::*, result::Error};
use dotenvy::dotenv;
use std::env::{self};
use self::models::{Card, NewCard};
pub mod hobbies;
pub mod models;
pub mod schema;
pub mod utils;
pub mod middlewares;

pub fn connect() -> PgConnection {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("Could not fetch database url");
    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}
pub fn create_yg_card(conn: &mut PgConnection, name: String, code: String, rarity: String, amount: String, edition: String, url: String) -> Card{
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
pub fn list_yg_cards(conn: &mut PgConnection) -> Json<Vec<Card>>{
    use self::schema::yg::dsl::*;

    let results =  yg
        .load::<Card>(conn)
        .expect("Error loading cards");
    Json(results)
}
pub fn delete_yg_card(conn: &mut PgConnection, target_id: &i32) -> usize{
    use self::schema::yg::dsl::*;

    let result = diesel::delete
        (yg.filter(id.eq(target_id)))
        .execute(conn).expect("Generic error");
    result
}
pub fn list_yg_card_by_id(conn: &mut PgConnection, target_id: &i32) -> Result<Json<Card>, Error>{
    use self::schema::yg::dsl::*;
    let count = yg.filter(id.eq(target_id)).count().get_result::<i64>(conn)?;
    if count == 0{
        return Err(diesel::NotFound)
    }
    let result = yg
        .filter(id.eq(target_id))
        .first::<Card>(conn)
        .expect("Could not find card by Id");
    
        Ok(Json(result))
    
}
pub fn create_pkmn_card() -> Card {
    todo!()
}
pub fn list_pkmn_cards() -> Json<Vec<Card>> {
    todo!()
}
pub fn delete_pkmn_card(){
    todo!()
}
pub fn list_pkmn_card_by_id() -> Json<Card>{
    todo!()
}