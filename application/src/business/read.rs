// application/src/test/read.rs

use domain::models::Business;
use domain::modelsext::BusinessCreated;
use infrastructure as db;
use diesel::prelude::*;
use shared::error_handler::CustomError;

pub fn get_business(api: BusinessCreated) -> Result<Business, CustomError> {
    use domain::schema::businesses::dsl::*;

    let mut conn = db::connection()?;

    let business = businesses.filter(api_id.eq(api.api_id).and(api_secret.eq(api.api_secret))).first::<Business>(&mut conn)?;
    Ok(business)
}

pub fn list_business(model_id: i32) -> Result<Business, CustomError> {
    use domain::schema::businesses;

    let mut conn = db::connection()?;

    let business = businesses::table.find(model_id).first::<Business>(&mut conn)?;
    Ok(business)
}

pub fn list_businesses() -> Result<Vec<Business>, CustomError> {
    use domain::schema::businesses;

    let mut conn = db::connection()?;

    let mut business = businesses::table.select(businesses::all_columns).load::<Business>(&mut conn)?;
    business.sort();
    Ok(business)
}