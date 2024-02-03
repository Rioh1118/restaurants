-- Your SQL goes here
CREATE TABLE stations (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    latitude FLOAT NOT NULL,
    longitude FLOAT NOT NULL
);