// application/src/business/read.rs

use domain::models::Business;
use domain::modelsext::BusinessCreated;
use infrastructure as db;
use diesel::prelude::*;
use shared::error_handler::CustomError;


pub fn list_business(model_id: i32) -> Result<Business, CustomError> {
    use domain::schema::businesses;

    let mut conn = db::connection()?;

    if model_id.clone() <= 0 {
        dbg!("Error, id is required and must not be empty");
        return Err(CustomError::new(400, "id is required and must not be empty".to_string()));
    } 

    let business = businesses::table.find(model_id).first::<Business>(&mut conn)?;

    drop(conn);

    Ok(business)
}

pub fn list_businesses() -> Result<Vec<Business>, CustomError> {
    use domain::schema::businesses;

    let mut conn = db::connection()?;

    let mut business = businesses::table.select(businesses::all_columns).load::<Business>(&mut conn)?;
    business.sort();
 
    drop(conn);

    Ok(business)
}

pub fn get_business(api: BusinessCreated) -> Result<Business, CustomError> {
    use domain::schema::businesses::dsl::*;

    let mut conn = db::connection()?;

    if api.api_id.clone().trim().len() == 0 {
        dbg!("Error, api_id is required and must not be empty");        
        return Err(CustomError::new(400, "api_id is required and must not be empty".to_string()));
    } 
    if api.api_secret.clone().trim().len() == 0 {
        dbg!("Error, api_secret is required and must not be empty");        
        return Err(CustomError::new(400, "api_secret is required and must not be empty".to_string()));
    } 

    let business = businesses.filter(api_id.eq(api.api_id).and(api_secret.eq(api.api_secret))).first::<Business>(&mut conn)?;

    drop(conn);

    Ok(business)
}