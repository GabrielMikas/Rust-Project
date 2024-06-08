use models::{Card, NewCard};
use rocket::{post, serde::json::Json};
use rocket_testing::*;
#[macro_use] extern crate rocket;



#[post("/register", data = "<new_yg_card>")]
fn post_yg_card(new_yg_card: Json<NewCard<'_>>){
    let mut conn = connect();
    create_card(&mut conn, 
        new_yg_card.card_name,
        new_yg_card.card_code,
        new_yg_card.card_rarity,
        new_yg_card.card_amount,
        new_yg_card.card_edition,
        new_yg_card.card_url);

    
}
#[get("/list")]
fn list_yg_cards() -> Json<Vec<Card>>{
    let mut conn = connect();
    list_cards(&mut conn)
}
#[delete("/delete/<target_id>")]
fn delete_yg_card(target_id: i32){
    let mut conn = connect();
    delete_card(&mut conn, &target_id)
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/yg", routes![post_yg_card, list_yg_cards, delete_yg_card])
}