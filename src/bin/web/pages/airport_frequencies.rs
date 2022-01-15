use super::{AppState, ErrorResponse};
use actix_web::{get, web, HttpResponse, Responder};
use ourairports_api::ourairports::airport_frequencies::AirportFrequency;
use ourairports_api::ourairports::Id;

// TODO: query params:
// airport_ref
// airport_ident

#[get("/api/v1/airport-frequencies")]
pub async fn get_airport_frequencies(data: web::Data<AppState>) -> impl Responder {
    let body = &data
        .airport_frequencies
        .values()
        .collect::<Vec<&AirportFrequency>>();
    HttpResponse::Ok().json(body)
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
