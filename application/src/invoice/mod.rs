// application/src/post/mod.rs

use domain::models::{Business, BusinessNode};
use infrastructure as db;
use diesel::prelude::*;
use shared::error_handler::CustomError;


pub mod read;
pub mod create; 
pub mod update;
pub mod delete; 


pub async fn config_node(api_secret: &String) -> Result<BusinessNode, CustomError> {
    use domain::schema::businesses;
    use domain::schema::businesses_nodes;

    let mut conn = db::connection()?;

    let business = businesses::table.filter(businesses::api_secret.eq(api_secret)).select(Business::as_select()).get_result(&mut conn)?;
    //let businessnode = BusinessNode::belonging_to(&business).select(BusinessNode::as_select()).load(&mut conn)?;
    let businessnode = businesses_nodes::table.filter(businesses_nodes::business_id.eq(&business.id)).first::<BusinessNode>(&mut conn)?;
  
    Ok(businessnode)
}