use crate::schema::{restaurants, reviews,categories, stations};
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Restaurant {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
    pub rating: Option<f64>,
    pub status: String,
    pub place_id: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="restaurants"]
pub struct NewRestaurant {
    pub name: String,
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
    pub rating: Option<f64>,
    pub status: String,
    pub place_id: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Review {
    pub id: i32,
    pub restaurant_id: i32,
    pub author_name: String,
    pub rating: f64,
    pub text: Option<String>,
    pub time: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name="reviews"]
pub struct NewReview {
    pub restaurant_id: i32,
    pub author_name: String,
    pub rating: f64,
    pub text: Option<String>,
    pub time: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Station {
    pub id: i32,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name="stations"]
pub struct NewStation {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name="categories"]
pub struct NewCategory {
    pub name: String,
}
