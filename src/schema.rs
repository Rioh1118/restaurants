// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    restaurants (id) {
        id -> Int4,
        name -> Varchar,
        address -> Text,
        latitude -> Float8,
        longitude -> Float8,
        rating -> Nullable<Float8>,
        status -> Varchar,
        place_id -> Varchar,
    }
}

diesel::table! {
    reviews (id) {
        id -> Int4,
        restaurant_id -> Int4,
        author_name -> Varchar,
        rating -> Float8,
        text -> Nullable<Text>,
        time -> Timestamp,
    }
}

diesel::table! {
    stations (id) {
        id -> Int4,
        name -> Varchar,
        latitude -> Float8,
        longitude -> Float8,
    }
}

diesel::joinable!(reviews -> restaurants (restaurant_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    restaurants,
    reviews,
    stations,
);
