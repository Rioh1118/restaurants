use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug)]
pub struct PlacesResponse {
    pub html_attributions: Vec<String>,
    pub next_page_token: Option<String>,
    pub results: Vec<Place>,
    pub status: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Place {
    pub business_status: String,
    pub geometry: Geometry,
    pub icon: String,
    pub name: String,
    pub opening_hours: Option<OpeningHours>,
    pub photos: Option<Vec<Photo>>,
    pub place_id: String,
    pub plus_code: Option<i32>,
    pub price_level: Option<i32>,
    pub rating: Option<f64>,
    pub types: Vec<String>,
    pub user_ratings_total: Option<i32>,
    pub vicinity: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Geometry {
    pub location: Location,
    pub viewport: Viewport,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Viewport {
    pub northeast: Location,
    pub southwest: Location,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpeningHours {
    pub open_now:bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Photo {
    pub height: i32,
    pub photo_reference: String,
    pub width: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlusCode {
    pub compound_code: Option<String>,
    pub global_code: String,
}