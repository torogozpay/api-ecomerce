// application/src/nodes/create.rs

use domain::models::{BusinessNode, NewBusinessNode};
use infrastructure as db;
use diesel::prelude::*;
use shared::error_handler::CustomError;

pub fn create_business_node(businessnode: NewBusinessNode) -> Result<BusinessNode, CustomError> {
    use domain::schema::businesses_nodes;

    let mut conn = db::connection()?;

    let businessnode = NewBusinessNode::from(businessnode);
    let businessnode = diesel::insert_into(businesses_nodes::table).values(&businessnode).get_result::<BusinessNode>(&mut conn)?; 
    Ok(businessnode)    
}