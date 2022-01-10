use super::{AppState, ErrorResponse};
use actix_web::{get, web, HttpResponse, Responder};
use ourairports_api::ourairports::airports::Airport;
use ourairports_api::ourairports::Id;

#[get("/api/v1/airports")]
pub async fn get_airports(data: web::Data<AppState>) -> impl Responder {
    let body = &data.airports.values().collect::<Vec<&Airport>>();
    HttpResponse::Ok().json(body)
}

#[get("/api/v1/airports/{id}")]
pub async fn get_airports_by_id(id: web::Path<Id>, data: web::Data<AppState>) -> impl Responder {
    if let Some(item) = data.airports.get(&id) {
        HttpResponse::Ok().json(item)
    } else {
        HttpResponse::NotFound().json(ErrorResponse::new("No airport with the specified ID."))
    }
}
