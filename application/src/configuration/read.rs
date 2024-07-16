// application/src/configuration/read.rs

use domain::models::Configuration;
use infrastructure as db;
use diesel::prelude::*;
use shared::error_handler::CustomError;

pub fn list_config() -> Result<Configuration, CustomError> {
    use domain::schema::configuration;

    let mut conn = db::connection()?;

    let config = configuration::table.first::<Configuration>(&mut conn)?;

    drop(conn);

    Ok(config)
}