pub mod business;
pub mod invoice;
pub mod nodes;


use domain::models::{Business, BusinessNode};
use infrastructure as db;
use diesel::prelude::*;
use shared::error_handler::CustomError;
use actix_web::http::header::map::HeaderMap;


pub async fn verify_business(headers: &HeaderMap) -> Result<BusinessNode, CustomError> {
    let api_id= headers.get("api-id").unwrap().to_str().unwrap();
    let api_secret= headers.get("api-secret").unwrap().to_str().unwrap();
 
    let config = config_node(api_id, api_secret).await?;

    Ok(config)
 }
 
pub async fn config_node(api_id: &str, api_secret: &str) -> Result<BusinessNode, CustomError> {
    use domain::schema::businesses;
    use domain::schema::businesses_nodes;

    let mut conn = db::connection()?;

    let business = businesses::table.filter(businesses::api_id.eq(api_id).and(businesses::api_secret.eq(api_secret))).select(Business::as_select()).get_result(&mut conn)?;
    let businessnode = businesses_nodes::table.filter(businesses_nodes::business_id.eq(&business.id)).first::<BusinessNode>(&mut conn)?;

    Ok(businessnode) 
}