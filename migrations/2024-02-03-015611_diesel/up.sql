-- Your SQL goes here
CREATE TABLE reviews (
    id SERIAL PRIMARY KEY,
    restaurant_id INTEGER NOT NULL,
    author_name VARCHAR NOT NULL,
    rating FLOAT NOT NULL,
    text TEXT,
    time TIMESTAMP NOT NULL,
    FOREIGN KEY (restaurant_id) REFERENCES restaurants(id)
);
