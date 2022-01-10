use std::collections::HashMap;
use actix_web::{get, HttpResponse, Responder};
use serde::Serialize;
use ourairports_api::ourairports::countries::{Country, get_countries_csv};
use ourairports_api::ourairports::regions::{Region, get_regions_csv};
use ourairports_api::ourairports::Id;

pub mod countries;
pub mod regions;

/// Home page.
#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("<h1>OurAirports API</h1>")
}

pub struct AppState {
    pub countries: HashMap<Id, Country>,
    pub regions: HashMap<Id, Region>
}

impl AppState {
    pub fn new() -> ourairports_api::ourairports::Result<Self> {
        Ok(AppState {
            countries: get_countries_csv()?,
            regions: get_regions_csv()?
        })
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

impl ErrorResponse {
    pub fn new(error: &str) -> Self {
        ErrorResponse { error: error.to_string() }
    }
}