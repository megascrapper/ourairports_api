use std::collections::HashMap;
use actix_web::{get, HttpResponse, Responder};
use ourairports_api::ourairports::countries::Country;
use ourairports_api::ourairports::Id;

pub mod countries;

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("<h1>OurAirports API</h1>")
}

pub struct AppState {
    pub countries: HashMap<Id, Country>
}