// application/src/test/delete.rs

use infrastructure as db;
use diesel::prelude::*;
use domain::models::Business;
use shared::error_handler::CustomError;

pub fn delete_business(model_id: i32) -> Result<Vec<Business>, CustomError> {
    use domain::schema::businesses::dsl::*;
    use domain::schema::businesses;

    let mut conn = db::connection()?;

    diesel::delete(businesses.filter(id.eq(model_id))).execute(&mut conn)?;

    let mut business = businesses::table.select(businesses::all_columns).load::<Business>(&mut conn)?;
    business.sort();
    Ok(business)
}