use std::collections::BTreeSet;

use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

use ourairports_api::ourairports::navaids::Navaid;
use ourairports_api::ourairports::Id;

use super::{AppState, ErrorResponse};

#[derive(Deserialize)]
pub struct QueryParams {
    pub filename: Option<String>,
    pub ident: Option<String>,
    pub iso_country: Option<String>,
    pub associated_airport: Option<String>,
}

#[get("/api/v1/navaids")]
pub async fn get_navaids(
    data: web::Data<AppState>,
    params: web::Query<QueryParams>,
) -> impl Responder {
    if params.filename.is_some()
        || params.ident.is_some()
        || params.iso_country.is_some()
        || params.associated_airport.is_some()
    {
        let mut body = BTreeSet::new();
        for navaid in data.navaids.values() {
            if let Some(filename) = &params.filename {
                if filename.to_ascii_lowercase() == navaid.filename().to_ascii_lowercase() {
                    body.insert(navaid);
                }
            }
            if let Some(ident) = &params.ident {
                if ident.to_ascii_lowercase() == navaid.ident().to_ascii_lowercase() {
                    body.insert(navaid);
                }
            }
            if let Some(iso_country) = &params.iso_country {
                if iso_country.to_ascii_lowercase() == navaid.iso_country().to_ascii_lowercase() {
                    body.insert(navaid);
                }
            }
            if let Some(associated_airport) = &params.associated_airport {
                if associated_airport.to_ascii_lowercase()
                    == navaid.associated_airport().to_ascii_lowercase()
                {
                    body.insert(navaid);
                }
            }
        }
        HttpResponse::Ok().json(body)
    } else {
        let body = data.navaids.values().collect::<Vec<&Navaid>>();
        HttpResponse::Ok().json(body)
    }
}

#[get("/api/v1/navaids/{id}")]
pub async fn get_navaids_by_id(id: web::Path<Id>, data: web::Data<AppState>) -> impl Responder {
    if let Some(item) = data.navaids.get(&id) {
        HttpResponse::Ok().json(item)
    } else {
        HttpResponse::NotFound().json(ErrorResponse::new("No navaid with the specified ID."))
    }
}
