// application/src/nodes/read.rs

use domain::models::BusinessNode;
use infrastructure as db;
use diesel::prelude::*;
use shared::error_handler::CustomError;

pub fn list_business_node(model_id: i32) -> Result<BusinessNode, CustomError> {
    use domain::schema::businesses_nodes;

    let mut conn = db::connection()?;

    let businessnode = businesses_nodes::table.find(model_id).first::<BusinessNode>(&mut conn)?;
    Ok(businessnode)
}

pub fn list_businesses_nodes() -> Result<Vec<BusinessNode>, CustomError> {
    use domain::schema::businesses_nodes;

    let mut conn = db::connection()?;

    let mut businessnode = businesses_nodes::table.select(businesses_nodes::all_columns).load::<BusinessNode>(&mut conn)?;
    businessnode.sort();
    Ok(businessnode)
}