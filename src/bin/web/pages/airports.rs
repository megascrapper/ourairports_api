use std::collections::BTreeSet;

use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

use ourairports_api::ourairports::airports::Airport;
use ourairports_api::ourairports::Id;

use super::{AppState, ErrorResponse};

#[derive(Deserialize)]
pub struct QueryParams {
    pub ident: Option<String>,
    pub iso_country: Option<String>,
    pub iso_region: Option<String>,
    pub gps_code: Option<String>,
    pub iata_code: Option<String>,
    pub local_code: Option<String>,
}

#[get("/api/v1/airports")]
pub async fn get_airports(
    data: web::Data<AppState>,
    params: web::Query<QueryParams>,
) -> impl Responder {
    if params.ident.is_some()
        || params.iso_country.is_some()
        || params.iso_region.is_some()
        || params.gps_code.is_some()
        || params.iata_code.is_some()
        || params.local_code.is_some()
    {
        let mut body = BTreeSet::new();
        for airport in data.airports.values() {
            if let Some(ident) = &params.ident {
                if ident.to_ascii_lowercase() == airport.ident().to_ascii_lowercase() {
                    body.insert(airport);
                }
            }
            if let Some(iso_country) = &params.iso_country {
                if iso_country.to_ascii_lowercase() == airport.iso_country().to_ascii_lowercase() {
                    body.insert(airport);
                }
            }
            if let Some(iso_region) = &params.iso_region {
                if iso_region.to_ascii_lowercase() == airport.iso_region().to_ascii_lowercase() {
                    body.insert(airport);
                }
            }
            if let Some(gps_code) = &params.gps_code {
                if gps_code.to_ascii_lowercase() == airport.gps_code().to_ascii_lowercase() {
                    body.insert(airport);
                }
            }
            if let Some(iata_code) = &params.iata_code {
                if iata_code.to_ascii_lowercase() == airport.iata_code().to_ascii_lowercase() {
                    body.insert(airport);
                }
            }
            if let Some(local_code) = &params.local_code {
                if local_code.to_ascii_lowercase() == airport.local_code().to_ascii_lowercase() {
                    body.insert(airport);
                }
            }
        }
        HttpResponse::Ok().json(body)
    } else {
        let body = data.airports.values().collect::<Vec<&Airport>>();
        HttpResponse::Ok().json(body)
    }
}

#[get("/api/v1/airports/{id}")]
pub async fn get_airports_by_id(id: web::Path<Id>, data: web::Data<AppState>) -> impl Responder {
    if let Some(item) = data.airports.get(&id) {
        HttpResponse::Ok().json(item)
    } else {
        HttpResponse::NotFound().json(ErrorResponse::new("No airport with the specified ID."))
    }
}
