pub mod models;
pub mod error;

use dotenv::dotenv;
use crate::api::api_client::api_keys::{get_api_key,Key,Api};
use models::*;
use error::PlacesError;

pub enum PlaceType {
    Restaurant,
    Cafe,
    Bar,
}

impl PlaceType {
    pub fn as_str(&self) -> &str {
        match self {
            PlaceType::Restaurant => "restaurant",
            PlaceType::Cafe => "cafe",
            PlaceType::Bar => "bar",
        }
    }
}


pub struct SearchQuery {
    pub place_type: PlaceType,
    pub location: (f64,f64), // (latitude, longitude)
    pub radius: u32,
    pub keyword: Option<String>, // If you specify more than one keyword, separate them with a space.
    pub language: Option<String>,
}


pub async fn fetch_places(search_query: &SearchQuery) -> Result<PlacesResponse,PlacesError>{
    
    dotenv().ok();
    let api_key = match get_api_key(Api::GooglePlaces) {
        Ok(Key::GooglePlaces(key)) => key,
        Err(e) => return Err(e.into()),
    };

    let url = format!("https://maps.googleapis.com/maps/api/place/nearbysearch/json?location={},{}&radius={}&type={}&keyword={}&language={}&key={}",
        search_query.location.0,
        search_query.location.1,
        search_query.radius,
        search_query.place_type.as_str(),
        search_query.keyword.as_ref().unwrap_or(&"".to_string()),
        search_query.language.as_ref().unwrap_or(&"ja".to_string()),
        api_key
    );
    let client = reqwest::Client::new();

    let res = client.get(url).send().await?.json::<PlacesResponse>().await?;
    Ok(res)
}

