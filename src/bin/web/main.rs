use actix_web::{App, HttpServer, middleware};
use crate::pages::{AppState, index};
use crate::pages::countries::*;
use crate::pages::regions::*;

mod pages;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(move || {
        App::new()
            .data(AppState::new().expect("error fetching OurAirports data"))
            .wrap(middleware::Compress::default()) // to enable compression
            .service(index)
            .service(get_countries)
            .service(get_countries_by_id)
            .service(get_regions)
            .service(get_regions_by_id)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}