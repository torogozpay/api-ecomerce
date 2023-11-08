// application/src/nodes/delete.rs

use infrastructure as db;
use diesel::prelude::*;
use domain::models::BusinessNode;
use shared::error_handler::CustomError;

pub fn delete_business_node(model_id: i32) -> Result<Vec<BusinessNode>, CustomError> {
    use domain::schema::businesses_nodes::dsl::*;
    use domain::schema::businesses_nodes;

    let mut conn = db::connection()?;

    diesel::delete(businesses_nodes.filter(id.eq(model_id))).execute(&mut conn)?;

    let mut businessnode = businesses_nodes::table.select(businesses_nodes::all_columns).load::<BusinessNode>(&mut conn)?;
    businessnode.sort();
    Ok(businessnode)
}