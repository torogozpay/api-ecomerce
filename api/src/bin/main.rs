// api/src/bin/main.rs
#![allow(non_snake_case)]

use actix_web::{web, App, HttpServer};
use listenfd::ListenFd;

use infrastructure as db;
use api::{jwt_handler,business_handler,invoice_handler,test_handler,swagger};
use shared::settings::CONFIG;

use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use std::env;

/* 
use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::{embed_migrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../infrastructure/migrations");
*/

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(web::scope("/api/v1")
        .service(test_handler::get_test_handler)
        .service(test_handler::get_price_bitcoin_handler)
        .service(jwt_handler::generate_access_token_handler)
        .service(business_handler::list_businesses_handler)
        .service(business_handler::list_business_by_id_handler)
        .service(business_handler::list_business_by_api_handler)
        .service(business_handler::create_business_handler)
        .service(business_handler::update_business_handler)
        .service(business_handler::delete_business_handler)
        .service(business_handler::generate_data_api_handler)
        .service(invoice_handler::list_invoice_handler)
        .service(invoice_handler::save_preorder_handler)
    );
}



fn set_routes(config: &mut web::ServiceConfig) {
    swagger::init_swagger(config);
    init_routes(config);
}

/*
type DB=diesel::pg::Pg;
fn run_migrations(connection: &mut impl MigrationHarness<DB>) {
    let _=connection.run_pending_migrations(MIGRATIONS);
} 
*/

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", CONFIG.log.level.clone());

    // Tracing using RUST_LOG
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
 
    db::init();

    //let mut conn = db::connection().expect("database connection");
    //run_migrations(&mut conn);

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || App::new().configure(set_routes));

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = CONFIG.server.host.clone();
            let port = CONFIG.server.port.clone();
            server.bind(format!("{host}:{port}"))?
        }
    };

    println!("🚀 API Ecommerce started successfully");

    server.run().await
}