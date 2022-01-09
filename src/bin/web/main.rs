use ourairports_api::ourairports::countries::get_countries_csv;
use actix_web::{App, HttpServer};
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
            .service(get_countries_by_id)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}