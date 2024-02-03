use dotenv::dotenv;
use std::env;
pub enum Key {
    GooglePlaces(String),
}

pub enum Api {
    GooglePlaces,
}

pub fn get_api_key (api: Api) -> Result<Key, std::env::VarError> {
    dotenv().ok();
    match api {
        Api::GooglePlaces => {
            let key = env::var("GOOGLE_PLACES_API_KEY")?;
            Ok(Key::GooglePlaces(key))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_api_key() {
        env::set_var("GOOGLE_PLACES_API_KEY", "test_key");

        let result = get_api_key(Api::GooglePlaces);

        match result {
            Ok(Key::GooglePlaces(result)) => assert_eq!(result, "test_key"),
            _ => panic!("Unexpected result"),
        }
    }  
}