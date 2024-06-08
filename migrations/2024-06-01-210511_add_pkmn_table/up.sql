-- Your SQL goes here
CREATE TABLE pkmn (
    id SERIAL PRIMARY KEY,
    card_name VARCHAR(255) NOT NULL,
    card_code VARCHAR(255) NOT NULL,
    card_rarity VARCHAR(255) NOT NULL,
    card_amount VARCHAR(255) NOT NULL,
    card_edition VARCHAR(255) NOT NULL,
    card_url VARCHAR(255) NOT NULL
)