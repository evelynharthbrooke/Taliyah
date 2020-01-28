use serde::Deserialize;

use reqwest::blocking::Client;
use reqwest::Error;

use std::env;

#[derive(Debug, Deserialize)]
pub struct Response {
    pub results: Vec<ResultResponse>,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct ResultResponse {
    pub address_components: Vec<AddressComponent>,
    pub formatted_address: String,
    pub geometry: Geometry,
    pub place_id: String,
    pub types: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct AddressComponent {
    pub long_name: String,
    pub short_name: String,
    pub types: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Geometry {
    pub location: Location,
    pub location_type: String,
}

#[derive(Debug, Deserialize)]
pub struct Location {
    #[serde(rename = "lat")]
    pub latitude: f64,
    #[serde(rename = "lng")]
    pub longitude: f64,
}

/// Gets coordinates for a specified location using the Google Maps Geocoding API.
pub fn get_coordinates(location: String) -> Result<Response, Error> {
    let user_agent: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
    let api_key = env::var("GOOGLE_KEY").expect("Unable to read the provided Google API key.").to_string();
    let client = Client::builder().user_agent(user_agent).build()?;
    let url = "https://maps.googleapis.com/maps/api/geocode/json";
    let request: Response = client.get(url).query(&[("address", location), ("key", api_key)]).send()?.json()?;
    return Ok(request);
}
