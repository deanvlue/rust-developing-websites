-- Your SQL goes here
CREATE TABLE cats(
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    image_path VARCHAR NOT NULL
)