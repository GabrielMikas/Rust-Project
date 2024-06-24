-- Your SQL goes here
CREATE TABLE investments (
    id SERIAL PRIMARY KEY,
    investment_name VARCHAR(255) NOT NULL,
    investment_amount VARCHAR(255) NOT NULL,
    investment_rate VARCHAR(255) NOT NULL,
    investment_type VARCHAR(255) NOT NULL,
    created_at VARCHAR(255) NOT NULL,
    expires_at VARCHAR(255) NOT NULL
)