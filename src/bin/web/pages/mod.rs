use actix_web::http::header::LOCATION;
use actix_web::{get, HttpResponse, Responder};
use log::info;
use ourairports_api::ourairports::airport_frequencies::{
    get_airport_frequencies_csv, AirportFrequency,
};
use ourairports_api::ourairports::airports::{get_airports_csv, Airport};
use ourairports_api::ourairports::countries::{get_countries_csv, Country};
use ourairports_api::ourairports::navaids::{get_navaids_csv, Navaid};
use ourairports_api::ourairports::regions::{get_regions_csv, Region};
use ourairports_api::ourairports::runways::{get_runways_csv, Runway};
use ourairports_api::ourairports::{FetchError, Id};
use serde::Serialize;
use std::collections::BTreeMap;

pub mod airport_frequencies;
pub mod airports;
pub mod countries;
pub mod navaids;
pub mod regions;
pub mod runways;

/// Home page.
#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::MovedPermanently()
        .set_header(LOCATION, "/index.html")
        .finish()
}

#[derive(Clone)]
pub struct AppState {
    pub airports: BTreeMap<Id, Airport>,
    pub airport_frequencies: BTreeMap<Id, AirportFrequency>,
    pub runways: BTreeMap<Id, Runway>,
    pub navaids: BTreeMap<Id, Navaid>,
    pub countries: BTreeMap<Id, Country>,
    pub regions: BTreeMap<Id, Region>,
}

impl AppState {
    pub fn new() -> Result<Self, FetchError> {
        info!("Downloading OurAirports data");
        Ok(AppState {
            airports: get_airports_csv()?,
            airport_frequencies: get_airport_frequencies_csv()?,
            runways: get_runways_csv()?,
            navaids: get_navaids_csv()?,
            countries: get_countries_csv()?,
            regions: get_regions_csv()?,
        })
    }
}

#[derive(Serialize)]
/// For encoding error responses as JSON
pub struct ErrorResponse {
    error: String,
}

impl ErrorResponse {
    pub fn new(error: &str) -> Self {
        ErrorResponse {
            error: error.to_string(),
        }
    }
}
