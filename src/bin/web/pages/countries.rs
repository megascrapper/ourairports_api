use actix_web::{get, web, HttpResponse, Responder};
use ourairports_api::ourairports::Id;
use ourairports_api::ourairports::countries::Country;
use super::{AppState, ErrorResponse};

#[get("/api/v1/countries")]
pub async fn get_countries(data: web::Data<AppState>) -> impl Responder {
    let body = &data.countries.values().collect::<Vec<&Country>>();
    HttpResponse::Ok().json(body)
}

#[get("/api/v1/countries/{id}")]
pub async fn get_countries_by_id(web::Path(id): web::Path<Id>, data: web::Data<AppState>) -> impl Responder {
    if let Some(item) = data.countries.get(&id) {
        HttpResponse::Ok().json(item)
    } else {
        HttpResponse::NotFound().json(ErrorResponse::new("No country with the specified ID."))
    }
}