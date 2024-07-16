// api/src/bin/main.rs
#![allow(non_snake_case)]

use actix_web::{web, App, HttpServer};
use listenfd::ListenFd;

use infrastructure::{self as db};
use api::{jwt_handler,business_handler,invoice_handler,test_handler,swagger};
use shared::settings::CONFIG;


use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::{embed_migrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../infrastructure/migrations");

use tracing_appender::rolling::{Rotation, RollingFileAppender};
use tracing::info;
use std::env;

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(web::scope("/api/ecommerce/v1")
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
        .service(invoice_handler::list_order_handler)
        .service(invoice_handler::list_invoice_handler)
        .service(invoice_handler::save_preorder_handler)
    );
}


fn set_routes(config: &mut web::ServiceConfig) {
    let cnf = CONFIG.openapi.swagger.clone();
    if cnf {
        swagger::init_swagger(config);
    }
    init_routes(config);
}


type DB=diesel::pg::Pg;
fn run_migrations(connection: &mut impl MigrationHarness<DB>) {
    let _=connection.run_pending_migrations(MIGRATIONS);
} 



#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Tracing using RUST_LOG
    env::set_var("RUST_LOG", CONFIG.log.level.clone());

    // Configuring tracing appender
    let file_appender = RollingFileAppender::builder()
    .rotation(Rotation::DAILY) // rotate log files once per day
    .filename_prefix("API_Ecommerce.logging") // log files will have names like "xxx.logging.2024-01-09"
    .build("./logs") // write log files to the '/logs' directory
    .expect("failed to initialize rolling file appender");
    
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .init();

    // Configuring migrations
    db::init();
    let mut conn = db::connection().expect("database connection");
    run_migrations(&mut conn);

    // Configuring server
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || 
        App::new()
            .configure(set_routes));

    server = match listenfd.take_tcp_listener(0) { 
        Ok(Some(listener)) => server.listen(listener).expect("There is no listener"), 
        Ok(None) => {
            let host = CONFIG.server.host.clone();
            let port = CONFIG.server.port.clone();
            server.bind(format!("{host}:{port}")).expect("There is no  host and port")  
        },
        Err(_) => panic!("Unable to start API Ecommerce")
    };

    info!("🚀 API Ecommerce started successfully");

    let _ = server.run().await;

    Ok(())
}