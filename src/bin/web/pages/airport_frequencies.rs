use std::collections::BTreeSet;

use actix_web::{get, HttpResponse, Responder, web};
use serde::Deserialize;

use ourairports_api::ourairports::airport_frequencies::AirportFrequency;
use ourairports_api::ourairports::Id;

use super::{AppState, ErrorResponse};

#[derive(Deserialize)]
pub struct QueryParams {
    pub airport_ref: Option<Id>,
    pub airport_ident: Option<String>
}

#[get("/api/v1/airport-frequencies")]
pub async fn get_airport_frequencies(data: web::Data<AppState>, params: web::Query<QueryParams>) -> impl Responder {
    if params.airport_ref.is_some() || params.airport_ident.is_some() {
        let mut body = BTreeSet::new();
        for freq in data.airport_frequencies.values() {
            if let Some(airport_ref) = params.airport_ref {
                if airport_ref == freq.airport_ref() {
                    body.insert(freq);
                }
            }
            if let Some(airport_ident) = &params.airport_ident {
                if airport_ident.to_ascii_lowercase() == freq.airport_ident().to_ascii_lowercase() {
                    body.insert(freq);
                }
            }
        }
        HttpResponse::Ok().json(body)
    } else {
        let body = &data
            .airport_frequencies
            .values()
            .collect::<Vec<&AirportFrequency>>();
        HttpResponse::Ok().json(body)
    }
}

#[get("/api/v1/airport-frequencies/{id}")]
pub async fn get_airport_frequencies_by_id(
    id: web::Path<Id>,
    data: web::Data<AppState>,
) -> impl Responder {
    if let Some(item) = data.airport_frequencies.get(&id) {
        HttpResponse::Ok().json(item)
    } else {
        HttpResponse::NotFound().json(ErrorResponse::new("No airport with the specified ID."))
    }
}
