use std::collections::HashMap;
use std::sync::{mpsc, Mutex};
use std::thread;
use std::time::Duration;
use ourairports_api::ourairports::countries::{Country, get_countries_csv};
use ourairports_api::ourairports::Id;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

struct AppState {
    countries: HashMap<Id, Country>
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("<h1>OurAirports API</h1>")
}

#[get("/api/v1/countries")]
async fn get_countries(data: web::Data<AppState>) -> impl Responder {
    let body = &data.countries.values().collect::<Vec<&Country>>();
    HttpResponse::Ok().json(body)
}

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