// application/src/invoice/update.rs

use domain::models::{BusinessNode,Invoice, InvoiceDet, NewInvoiceDet, MyInvoice};
use infrastructure as db;
use diesel::prelude::*;
use shared::error_handler::CustomError;
use super::config_node;

pub async fn send_payment(mut myinvoice: MyInvoice,  payment_address: String) -> Result<MyInvoice, CustomError> {
    let _config = config_node(&myinvoice.api_secret).await?;
    myinvoice.master.payment_address = Some(payment_address);
    myinvoice.master.payment_status = Some("Paid".to_owned());
    let invoice = update_invoice(myinvoice).await?;

    Ok(invoice)
}

pub async fn update_invoice(myinvoice: MyInvoice) -> Result<MyInvoice, CustomError> {
    use domain::schema::invoices::dsl::*;
    use domain::schema::invoices_det::dsl::*;
    use domain::schema::invoices;
    use domain::schema::invoices_det;

    let mut conn = db::connection()?;

    let mut myinvoice = myinvoice;
    myinvoice.master.updated_at = Some(chrono::offset::Utc::now());
    let id_invoice = myinvoice.master.id;
 
    let newinvoice = diesel::update(invoices.filter(invoices::id.eq(id_invoice))).set(&myinvoice.master).get_result::<Invoice>(&mut conn)?;
    
    /*Procesando detalles de factura*/
    let mut list_insert: Vec<NewInvoiceDet> = Vec::new();
    let mut list_update: Vec<InvoiceDet> = Vec::new();
    let mut list_delete: Vec<InvoiceDet> = Vec::new();

    /*Detalle de facturas en la BD */
    let invoicetemps = invoices_det::table.filter(invoice_id.eq(id_invoice)).get_results::<InvoiceDet>(&mut conn)?;

    /*Identificando y borrando detalles de factura eliminados por usuario*/
    for det in invoicetemps {
        let mut result = true;
        for el in &myinvoice.details {
            if el.id == det.id {
                result = false;
            }        
        }

        if result {
            list_delete.push(det);
        }
    }
     
    for det in list_delete {
        diesel::delete(invoices_det.filter(invoices_det::id.eq(det.id))).execute(&mut conn)?;
    }
    
    /*Identificando y actualizando detalles de factura*/
    for element in myinvoice.details {
        if element.id > 0 {
            let mut row = InvoiceDet::from(element);
            row.invoice_id = newinvoice.id;
            list_update.push(row);
        } else {
            let row = NewInvoiceDet::new(newinvoice.id, element.product_code,element.quantity,element.amount);
            list_insert.push(row);
        }           
    } 

    for det in list_insert {
        diesel::insert_into(invoices_det::table).values(&det).get_result::<InvoiceDet>(&mut conn)?;
    }
    for det in list_update {
        diesel::update(invoices_det.filter(invoices_det::id.eq(det.id))).set(&det).get_result::<InvoiceDet>(&mut conn)?;
    }
 

    let invoicedets = InvoiceDet::belonging_to(&newinvoice).select(InvoiceDet::as_select()).load(&mut conn)?;
    let myresult = MyInvoice { api_secret: (myinvoice.api_secret), master: (newinvoice), details: (invoicedets) };

    Ok(myresult)            
}
