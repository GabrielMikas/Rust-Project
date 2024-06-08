use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::yg)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Card{
    pub id: i32,
    pub card_name: String,
    pub card_code: String,
    pub card_rarity: String,
    pub card_amount: String,
    pub card_edition: String,
    pub card_url: String,
}
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::yg)]
pub struct NewCard<'a>{
    pub card_name: &'a str,
    pub card_code: &'a str,
    pub card_rarity: &'a str,
    pub card_amount: &'a str,
    pub card_edition: &'a str,
    pub card_url: &'a str,
}