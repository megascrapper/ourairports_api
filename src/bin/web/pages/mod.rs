use actix_web::{get, HttpResponse, Responder};
use ourairports_api::ourairports::airports::{get_airports_csv, Airport};
use ourairports_api::ourairports::countries::{get_countries_csv, Country};
use ourairports_api::ourairports::regions::{get_regions_csv, Region};
use ourairports_api::ourairports::Id;
use serde::Serialize;
use std::collections::HashMap;

pub mod airports;
pub mod countries;
pub mod regions;

/// Home page.
#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("<h1>OurAirports API</h1>")
}

pub struct AppState {
    pub airports: HashMap<Id, Airport>,
    pub countries: HashMap<Id, Country>,
    pub regions: HashMap<Id, Region>,
}

impl AppState {
    pub fn new() -> ourairports_api::ourairports::Result<Self> {
        Ok(AppState {
            airports: get_airports_csv()?,
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
