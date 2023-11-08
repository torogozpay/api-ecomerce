// application/src/invoice/delete.rs

use infrastructure as db;
use diesel::prelude::*;
use domain::models::Invoice;
use shared::error_handler::CustomError;

pub fn delete_invoice(model_id: i32) -> Result<Vec<Invoice>, CustomError> {
    use domain::schema::invoices::dsl::*;
    use domain::schema::invoices_det::dsl::*;
    use domain::schema::invoices;

    let mut conn = db::connection()?;
    
    let num_deleted = diesel::delete(invoices_det.filter(invoice_id.eq(model_id))).execute(&mut conn)?;
    
    if num_deleted > 0 {
        diesel::delete(invoices.filter(invoices::id.eq(model_id))).execute(&mut conn)?;
    }
    
    let mut invoice = invoices::table.select(invoices::all_columns).load::<Invoice>(&mut conn)?;
    invoice.sort();
    
    Ok(invoice) 
}  