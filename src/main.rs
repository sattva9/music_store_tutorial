use crate::http_handlers::*;
use actix_web::web::ServiceConfig;
use actix_web::{web, App, HttpServer};

mod db;
mod http_handlers;
mod music_error;

#[tokio::main]
async fn main() {
    if let Err(e) = start_server().await {
        println!("Error while starting http server {}", e);
    }
}

async fn start_server() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8000/");
    HttpServer::new(|| App::new().configure(routes))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}

fn routes(service_config: &mut ServiceConfig) {
    service_config
        .route("/", web::get().to(welcome))
        .route("/list_music", web::get().to(list_music))
        .route("/add_music", web::get().to(add_music))
        .route("/add_music_to_db", web::post().to(add_music_to_db));
}
