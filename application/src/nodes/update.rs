// application/src/nodes/update.rs

use domain::models::BusinessNode;
use infrastructure as db;
use diesel::prelude::*;
use shared::error_handler::CustomError;

pub fn update_business_node(businessnode: BusinessNode) -> Result<BusinessNode, CustomError> {
    use domain::schema::businesses_nodes::dsl::*;

    let mut conn = db::connection()?;

    let id_node = businessnode.id;    
    let businessnode = diesel::update(businesses_nodes.filter(id.eq(id_node))).set(&businessnode).get_result::<BusinessNode>(&mut conn)?;
    Ok(businessnode)
}