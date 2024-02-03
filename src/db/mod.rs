
use diesel::prelude::*;
use crate::models::{Restaurant, NewRestaurant, Review, NewReview, Station, NewStation, Category};
use crate::schema::{restaurants::dsl::*, reviews::dsl::*};

pub fn create_restaurants(conn:&mut PgConnection, new_rest: NewRestaurant) -> QueryResult<Restaurant>{
    diesel::insert_into(restaurants)
        .values(&new_rest)
        .get_result(conn)
}
pub fn get_restaurants(conn:&mut PgConnection, rest_id: i32) -> QueryResult<Restaurant>{
    restaurants.find(rest_id).get_result::<Restaurant>(conn)
}
pub fn update_restaurant(conn: &mut PgConnection, rest_id: i32, rest_data: NewRestaurant) -> QueryResult<Restaurant> {
    use crate::schema::restaurants::dsl::*;

    diesel::update(restaurants.find(rest_id))
        .set((
            name.eq(rest_data.name),
            address.eq(rest_data.address),
            latitude.eq(rest_data.latitude),
            longitude.eq(rest_data.longitude),
            rating.eq(rest_data.rating),
            status.eq(rest_data.status),
            place_id.eq(rest_data.place_id),
        ))
        .get_result::<Restaurant>(conn)
}
pub fn delete_restaurant(conn: &mut PgConnection, rest_id: i32) -> QueryResult<usize> {

    diesel::delete(restaurants.find(rest_id)).execute(conn)
}

pub fn create_review(conn: &mut PgConnection, new_review: NewReview) -> QueryResult<Review> {

    diesel::insert_into(reviews)
        .values(&new_review)
        .get_result(conn)
}

pub fn get_category(conn: &mut PgConnection, category_id: i32) -> QueryResult<Category> {
    use crate::schema::categories::dsl::*;

    categories.find(category_id).get_result::<Category>(conn)
}

pub fn update_station(conn: &mut PgConnection, station_id: i32, station_data: NewStation) -> QueryResult<Station> {
    use crate::schema::stations::dsl::*;

    diesel::update(stations.find(station_id))
        .set((
            name.eq(station_data.name),
            latitude.eq(station_data.latitude),
            longitude.eq(station_data.longitude),
        ))
        .get_result::<Station>(conn)
}