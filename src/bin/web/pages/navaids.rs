use super::{AppState, ErrorResponse};
use actix_web::{get, web, HttpResponse, Responder};
use ourairports_api::ourairports::navaids::Navaid;
use ourairports_api::ourairports::Id;

#[get("/api/v1/navaids")]
pub async fn get_navaids(data: web::Data<AppState>) -> impl Responder {
    let body = &data.navaids.values().collect::<Vec<&Navaid>>();
    HttpResponse::Ok().json(body)
}

#[get("/api/v1/navaids/{id}")]
pub async fn get_navaids_by_id(id: web::Path<Id>, data: web::Data<AppState>) -> impl Responder {
    if let Some(item) = data.navaids.get(&id) {
        HttpResponse::Ok().json(item)
    } else {
        HttpResponse::NotFound().json(ErrorResponse::new("No navaid with the specified ID."))
    }
}
