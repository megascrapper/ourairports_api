use super::{AppState, ErrorResponse};
use actix_web::{get, web, HttpResponse, Responder};
use ourairports_api::ourairports::runways::Runway;
use ourairports_api::ourairports::Id;

// TODO: query params:
// airport_ref
// airport_ident

#[get("/api/v1/runways")]
pub async fn get_runways(data: web::Data<AppState>) -> impl Responder {
    let body = &data.runways.values().collect::<Vec<&Runway>>();
    HttpResponse::Ok().json(body)
}

#[get("/api/v1/runways/{id}")]
pub async fn get_runways_by_id(id: web::Path<Id>, data: web::Data<AppState>) -> impl Responder {
    if let Some(item) = data.runways.get(&id) {
        HttpResponse::Ok().json(item)
    } else {
        HttpResponse::NotFound().json(ErrorResponse::new("No runway with the specified ID."))
    }
}
