CREATE TABLE restaurants (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    address TEXT NOT NULL,
    latitude FLOAT NOT NULL,
    longitude FLOAT NOT NULL,
    rating FLOAT,
    status VARCHAR NOT NULL,
    place_id VARCHAR NOT NULL
);