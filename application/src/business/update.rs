// application/src/test/update.rs

use domain::models::Business;
use infrastructure as db;
use diesel::prelude::*;
use shared::error_handler::CustomError;

pub fn update_business(business: Business) -> Result<Business, CustomError> {
    use domain::schema::businesses::dsl::*;

    let mut conn = db::connection()?;

    let mut mybusiness = business;
    mybusiness.updated_at = Some(chrono::offset::Utc::now());
    let id_business = mybusiness.id;
    
    let business = diesel::update(businesses.filter(id.eq(id_business))).set(&mybusiness).get_result::<Business>(&mut conn)?;
    Ok(business)
}