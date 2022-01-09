use ourairports_api::ourairports::countries::get_countries_csv;
use ourairports_api::ourairports::Id;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::pages::{AppState, index};
use crate::pages::countries::*;

mod pages;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(AppState {
                countries: get_countries_csv().unwrap()
            })
            .service(index)
            .service(get_countries)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}