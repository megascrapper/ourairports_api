use actix_web::{get, HttpResponse, Responder};
use ourairports_api::ourairports::airport_frequencies::{get_airport_frequencies_csv, AirportFrequency};
use ourairports_api::ourairports::airports::{get_airports_csv, Airport};
use ourairports_api::ourairports::runways::{get_runways_csv, Runway};
use ourairports_api::ourairports::countries::{get_countries_csv, Country};
use ourairports_api::ourairports::regions::{get_regions_csv, Region};
use ourairports_api::ourairports::Id;
use serde::Serialize;
use std::collections::BTreeMap;

pub mod airport_frequencies;
pub mod airports;
pub mod countries;
pub mod regions;
pub mod runways;

/// Home page.
#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("<h1>OurAirports API</h1>")
}

pub struct AppState {
    pub airports: BTreeMap<Id, Airport>,
    pub airport_frequencies: BTreeMap<Id, AirportFrequency>,
    pub runways: BTreeMap<Id, Runway>,
    pub countries: BTreeMap<Id, Country>,
    pub regions: BTreeMap<Id, Region>,
}

impl AppState {
    pub fn new() -> ourairports_api::ourairports::Result<Self> {
        Ok(AppState {
            airports: get_airports_csv()?,
            airport_frequencies: get_airport_frequencies_csv()?,
            runways: get_runways_csv()?,
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
