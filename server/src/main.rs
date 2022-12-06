use actix_web::{web, App, HttpServer, middleware::Logger};
use dotenvy::dotenv;
use server::services;
use server::db;
use std::sync::Mutex;
use server::state::State;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Logging
    dotenv().ok();
    if std::env::var("MODE").unwrap() == "debug" {
        std::env::set_var("RUST_LOG", "debug");
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    env_logger::init();

    // Get port
    let port: u16 = std::env::var("PORT").unwrap().parse::<u16>().unwrap();

    // Show server data
    println!("[\x1b[32mINFO\x1b[37m] Server running on port: \x1b[34m{port}\x1b[37m; [\x1b[34mhttp://localhost:{port}\x1b[37m]");

    // Construct State 
    let state = web::Data::new(State {
        db: Mutex::new(db::init().await.unwrap())
    });

    // Create HttpServer
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
        .app_data(state.clone())
        .service(
           web::scope("/api")
            .service(
                web::scope("/profile")
                    .service(services::api::profile::list)
                    .service(services::api::profile::add)
                    .service(services::api::profile::delete)
                    .service(services::api::profile::update)
                    .service(services::api::profile::list_albums)
            )
            .service(
                web::scope("/album")
            )
        )
        .wrap(logger)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
