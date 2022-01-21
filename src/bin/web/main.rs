use crate::pages::airport_frequencies::*;
use crate::pages::airports::*;
use crate::pages::countries::*;
use crate::pages::navaids::*;
use crate::pages::regions::*;
use crate::pages::runways::*;
use crate::pages::{index, AppState};
use actix_files;
use actix_web::{middleware, App, HttpServer};
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
                    .wrap(middleware::Compress::default()) // to enable compression
                    .wrap(middleware::Logger::default())
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
                    .service(actix_files::Files::new("/", "./static").show_files_listing())
            })
            .bind(std::env::var("BIND_ADDRESS").unwrap_or("127.0.0.1:8080".to_string()))?
            .run()
            .await
        }
        Err(e) => {
            error!("{}", e);
            error!("fatal error: cannot fetch OurAirports data. stopping immediately");
            panic!("cannot fetch OurAirports data: {}", e);
        }
    }
}
