pub mod business;
pub mod invoice;
pub mod bitcoin;


use domain::models::Business;
use infrastructure as db;
use diesel::prelude::*;
use shared::error_handler::CustomError;
use actix_web::http::header::map::HeaderMap;


pub async fn verificate_business(headers: &HeaderMap) -> Result<Business, CustomError> {
    let api_id: String = headers.get("api-id").unwrap().to_str().unwrap().to_string();
    let api_secret: String = headers.get("api-secret").unwrap().to_str().unwrap().to_string();
 
    let config = config_business(api_id, api_secret).await?;

    Ok(config)
 }
 
pub async fn config_business(api_id: String, api_secret: String) -> Result<Business, CustomError> {
    use domain::schema::businesses;

    let mut conn = db::connection()?;

    let business = businesses::table.filter(businesses::api_id.eq(api_id).and(businesses::api_secret.eq(api_secret))).select(Business::as_select()).get_result(&mut conn)?;
 
    Ok(business) 
}

