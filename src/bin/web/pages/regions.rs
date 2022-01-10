use super::{AppState, ErrorResponse};
use actix_web::{get, web, HttpResponse, Responder};
use ourairports_api::ourairports::regions::Region;
use ourairports_api::ourairports::Id;

#[get("/api/v1/regions")]
pub async fn get_regions(data: web::Data<AppState>) -> impl Responder {
    let body = &data.regions.values().collect::<Vec<&Region>>();
    HttpResponse::Ok().json(body)
}

#[get("/api/v1/regions/{id}")]
pub async fn get_regions_by_id(id: web::Path<Id>, data: web::Data<AppState>) -> impl Responder {
    if let Some(item) = data.regions.get(&id) {
        HttpResponse::Ok().json(item)
    } else {
        HttpResponse::NotFound().json(ErrorResponse::new("No region with the specified ID."))
    }
}
