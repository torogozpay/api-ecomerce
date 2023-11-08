// application/src/invoice/read.rs

use domain::models::{BusinessNode,Invoice, InvoiceDet, MyInvoice};
use infrastructure as db;
use diesel::prelude::*;
use shared::error_handler::CustomError;
//use anyhow::anyhow;

//use prost::Message; // for deserializing gRPC messages
//use serde_json::to_string_pretty; // for serializing to JSON
use reqwest::{header, Response};  


pub async fn list_invoice_by_id(model_id: i32) -> Result<MyInvoice, CustomError> {
    use domain::schema::invoices;

    let mut conn = db::connection()?;

    let invoice = invoices::table.filter(invoices::id.eq(model_id)).select(Invoice::as_select()).get_result(&mut conn)?;
    let invoicedets = InvoiceDet::belonging_to(&invoice).select(InvoiceDet::as_select()).load(&mut conn)?;
    let myresult = MyInvoice { api_secret: ("".to_owned()), master: (invoice), details: (invoicedets) };

    Ok(myresult)
}


pub fn list_invoices() -> Result<Vec<Invoice>, CustomError> {
    use domain::schema::invoices;

    let mut conn = db::connection()?;

    let mut invoices = invoices::table.select(invoices::all_columns).load::<Invoice>(&mut conn)?;
    invoices.sort();

    Ok(invoices)
}


pub async fn list_invoice_by_hash(config : BusinessNode, payment_hash : String) -> Result<Response, anyhow::Error> {
    let client = reqwest::Client::builder()
                .build()?;

    let query = vec![
        ("path", config.path),   
        ("payment_hash", payment_hash) //hex::decode(payment_hash).expect("Failed to decode payment hash"))     
    ];

    // Construct the request
    let response = client       
            .get("http://localhost:9898/get_invoice")
            //.header(header::AUTHORIZATION, format!("Bearer {}", token)) 
            .header(header::CONTENT_TYPE, "application/json") 
            .header(header::ACCEPT, "application/grpc")       
            .query(&query) 
            .send()
            .await?;

    // Parse and process the response
    if response.status().is_success() {
        // Serialize the gRPC response struct to JSON
        //let json_response = to_string_pretty(&response).unwrap();
        //println!("{:?}", json_response);
    } else {
        println!("Request failed with status: {}", response.status());
    } 

    Ok(response)    
}    