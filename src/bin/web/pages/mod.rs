use std::collections::HashMap;
use actix_web::{get, HttpResponse, Responder};
use serde::Serialize;
use ourairports_api::ourairports::countries::Country;
use ourairports_api::ourairports::Id;

pub mod countries;

/// Home page.
#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("<h1>OurAirports API</h1>")
}

pub struct AppState {
    pub countries: HashMap<Id, Country>
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