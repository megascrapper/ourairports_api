use std::collections::BTreeSet;

use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

use ourairports_api::ourairports::countries::Country;
use ourairports_api::ourairports::Id;

use super::{AppState, ErrorResponse};

#[derive(Deserialize)]
pub struct QueryParams {
    pub code: Option<String>,
    pub name: Option<String>,
}

#[get("/api/v1/countries")]
pub async fn get_countries(
    data: web::Data<AppState>,
    params: web::Query<QueryParams>,
) -> impl Responder {
    if params.name.is_some() || params.code.is_some() {
        let mut body = BTreeSet::new();
        for country in data.countries.values() {
            if let Some(name) = &params.name {
                if name.to_ascii_lowercase() == country.name().to_ascii_lowercase() {
                    body.insert(country);
                }
            }
            if let Some(code) = &params.code {
                if code.to_ascii_lowercase() == country.code().to_ascii_lowercase() {
                    body.insert(country);
                }
            }
        }
        HttpResponse::Ok().json(body)
    } else {
        let body = data.countries.values().collect::<Vec<&Country>>();
        HttpResponse::Ok().json(body)
    }
}

#[get("/api/v1/countries/{id}")]
pub async fn get_countries_by_id(id: web::Path<Id>, data: web::Data<AppState>) -> impl Responder {
    if let Some(item) = data.countries.get(&id) {
        HttpResponse::Ok().json(item)
    } else {
        HttpResponse::NotFound().json(ErrorResponse::new("No country with the specified ID."))
    }
}
