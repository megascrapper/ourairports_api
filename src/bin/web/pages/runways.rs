use std::collections::BTreeSet;
use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

use ourairports_api::ourairports::runways::Runway;
use ourairports_api::ourairports::Id;

use super::{AppState, ErrorResponse};

#[derive(Deserialize)]
pub struct QueryParams {
    pub airport_ref: Option<Id>,
    pub airport_ident: Option<String>,
}

#[get("/api/v1/runways")]
pub async fn get_runways(
    data: web::Data<AppState>,
    params: web::Query<QueryParams>,
) -> impl Responder {
    if params.airport_ref.is_some() || params.airport_ident.is_some() {
        let mut body = BTreeSet::new();
        for runway in data.runways.values() {
            if let Some(airport_ref) = params.airport_ref {
                if airport_ref == runway.airport_ref() {
                    body.insert(runway);
                }
            }
            if let Some(airport_ident) = &params.airport_ident {
                if airport_ident.to_ascii_lowercase() == runway.airport_ident().to_ascii_lowercase() {
                    body.insert(runway);
                }
            }
        }
        HttpResponse::Ok().json(body)
    } else {
        let body = &data.runways.values().collect::<Vec<&Runway>>();
        HttpResponse::Ok().json(body)
    }
}

#[get("/api/v1/runways/{id}")]
pub async fn get_runways_by_id(id: web::Path<Id>, data: web::Data<AppState>) -> impl Responder {
    if let Some(item) = data.runways.get(&id) {
        HttpResponse::Ok().json(item)
    } else {
        HttpResponse::NotFound().json(ErrorResponse::new("No runway with the specified ID."))
    }
}
