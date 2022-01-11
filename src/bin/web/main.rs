use crate::pages::airport_frequencies::*;
use crate::pages::airports::*;
use crate::pages::countries::*;
use crate::pages::navaids::*;
use crate::pages::regions::*;
use crate::pages::runways::*;
use crate::pages::{index, AppState};
use actix_web::{middleware, App, HttpServer};

mod pages;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .data(AppState::new().expect("error fetching OurAirports data"))
            .wrap(middleware::Compress::default()) // to enable compression
            .service(index)
            .service(get_airports)
            .service(get_airports_by_id)
            .service(get_airport_frequencies)
            .service(get_airport_frequencies_by_id)
            .service(get_runways)
            .service(get_runways_by_id)
            .service(get_navaids)
            .service(get_navaids_by_id)
            .service(get_countries)
            .service(get_countries_by_id)
            .service(get_regions)
            .service(get_regions_by_id)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
