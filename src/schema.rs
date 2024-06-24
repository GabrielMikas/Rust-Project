// @generated automatically by Diesel CLI.

diesel::table! {
    investments (id) {
        id -> Int4,
        #[max_length = 255]
        investment_name -> Varchar,
        #[max_length = 255]
        investment_amount -> Varchar,
        #[max_length = 255]
        investment_rate -> Varchar,
        #[max_length = 255]
        investment_type -> Varchar,
        #[max_length = 255]
        created_at -> Varchar,
        #[max_length = 255]
        expires_at -> Varchar,
    }
}

diesel::table! {
    pkmn (id) {
        id -> Int4,
        #[max_length = 255]
        card_name -> Varchar,
        #[max_length = 255]
        card_code -> Varchar,
        #[max_length = 255]
        card_rarity -> Varchar,
        #[max_length = 255]
        card_amount -> Varchar,
        #[max_length = 255]
        card_edition -> Varchar,
        #[max_length = 255]
        card_url -> Varchar,
    }
}

diesel::table! {
    yg (id) {
        id -> Int4,
        #[max_length = 255]
        card_name -> Varchar,
        #[max_length = 255]
        card_code -> Varchar,
        #[max_length = 255]
        card_rarity -> Varchar,
        #[max_length = 255]
        card_amount -> Varchar,
        #[max_length = 255]
        card_edition -> Varchar,
        #[max_length = 255]
        card_url -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    investments,
    pkmn,
    yg,
);
