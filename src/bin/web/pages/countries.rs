use actix_web::{get, web, HttpResponse, Responder};
use ourairports_api::ourairports::countries::Country;
use super::AppState;

#[get("/api/v1/countries")]
pub async fn get_countries(data: web::Data<AppState>) -> impl Responder {
    let body = &data.countries.values().collect::<Vec<&Country>>();
    HttpResponse::Ok().json(body)
}