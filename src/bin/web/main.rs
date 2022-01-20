use crate::pages::airport_frequencies::*;
use crate::pages::airports::*;
use crate::pages::countries::*;
use crate::pages::navaids::*;
use crate::pages::regions::*;
use crate::pages::runways::*;
use crate::pages::{index, AppState};
use actix_web::{middleware, App, HttpServer};
use actix_files;
use env_logger::Env;
use log::error;

mod pages;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    match AppState::new() {
        Ok(app_state) => {
            HttpServer::new(move || {
                App::new()
                    .data(app_state.clone())
                    .wrap(middleware::Compress::default())
                    .wrap(middleware::Logger::default())// to enable compression
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
                    //.service(actix_files::Files::new("/docs", "./static/docs"))
            })
            .bind("0.0.0.0:8080")?
            .run()
            .await
        }
        Err(e) => {
            error!("{}", e);
            error!("fatal error: cannot fetch OurAirports data. stopping immediately");
            panic!("cannot fetch OurAirports data: {}", e);
        },
    }
}
