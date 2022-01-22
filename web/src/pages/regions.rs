use std::collections::BTreeSet;

use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

use ourairports::regions::Region;
use ourairports::Id;

use super::{AppState, ErrorResponse};

#[derive(Deserialize)]
pub struct QueryParams {
    pub iso_country: Option<String>,
    pub code: Option<String>,
    pub local_code: Option<String>,
}

#[get("/api/v1/regions")]
pub async fn get_regions(
    data: web::Data<AppState>,
    params: web::Query<QueryParams>,
) -> impl Responder {
    if params.iso_country.is_some() || params.code.is_some() || params.local_code.is_some() {
        let mut body = BTreeSet::new();
        for region in data.regions.values() {
            if let Some(iso_country) = &params.iso_country {
                if iso_country.to_ascii_lowercase() == region.iso_country().to_ascii_lowercase() {
                    body.insert(region);
                }
            }
            if let Some(code) = &params.code {
                if code.to_ascii_lowercase() == region.code().to_ascii_lowercase() {
                    body.insert(region);
                }
            }
            if let Some(local_code) = &params.local_code {
                if local_code.to_ascii_lowercase() == region.local_code().to_ascii_lowercase() {
                    body.insert(region);
                }
            }
        }
        HttpResponse::Ok().json(body)
    } else {
        let body = data.regions.values().collect::<Vec<&Region>>();
        HttpResponse::Ok().json(body)
    }
}

#[get("/api/v1/regions/{id}")]
pub async fn get_regions_by_id(id: web::Path<Id>, data: web::Data<AppState>) -> impl Responder {
    if let Some(item) = data.regions.get(&id) {
        HttpResponse::Ok().json(item)
    } else {
        HttpResponse::NotFound().json(ErrorResponse::new("No region with the specified ID."))
    }
}
