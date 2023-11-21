// application/src/invoice/read.rs

use lazy_static::lazy_static;

use domain::models::{BusinessNode,Invoice, InvoiceDet, MyInvoice};
use domain::modelsext::ListInvoicesExt;
use infrastructure as db;
use diesel::prelude::*;
use shared::error_handler::CustomError;
//use anyhow::anyhow;
use reqwest::{header, Client};

use shared::settings;

lazy_static! {
    static ref CONFIG: settings::Settings =
        settings::Settings::new().expect("Config can be loaded");
}


pub async fn list_invoice_by_id(model_id: i32) -> Result<MyInvoice, CustomError> {
    use domain::schema::invoices;

    let mut conn = db::connection()?;

    let invoice = invoices::table.filter(invoices::id.eq(model_id)).select(Invoice::as_select()).get_result(&mut conn)?;
    let invoicedets = InvoiceDet::belonging_to(&invoice).select(InvoiceDet::as_select()).load(&mut conn)?;
    let myresult = MyInvoice { master: (invoice), details: (invoicedets) };

    Ok(myresult)
}


pub async fn list_invoices(_config : &BusinessNode) -> Result<Vec<Invoice>, CustomError> {
    use domain::schema::invoices;
    use domain::schema::businesses_nodes;

    let mut conn = db::connection()?;

    let mut invoices = invoices::table.select(invoices::all_columns).load::<Invoice>(&mut conn)?;
    //.filter(businesses_nodes::business_id.eq(_config.business_id))
    //let business = businesses::table.filter(businesses::api_id.eq(api_id).and(businesses::api_secret.eq(api_secret))).select(Business::as_select()).get_result(&mut conn)?;

    invoices.sort();

    Ok(invoices)
}


pub async fn list_invoice_by_hash(_config : &BusinessNode, payment_hash : String) -> Result<ListInvoicesExt, CustomError> {
    let api_key = CONFIG.api.api_key.clone();
    let username = CONFIG.api.api_username.clone();
    let password = CONFIG.api.api_password.clone();
    let auth_string = format!("{}:{}", username, password);
    let auth_encoded = "Basic ".to_owned() + &base64::encode(&auth_string);

    let json = &serde_json::json!({
        //"path": _config.path.to_string(),
        "hash": payment_hash.to_string()
    });

    // Construct the request
    let client = Client::builder().build()?; 
    let response = client
            .get("https://6df3-138-186-251-29.ngrok-free.app/api/v1/getInvoice")
            .header("Authorization", format!("{}", auth_encoded))
            .header("x-api-key", api_key) 
            .header(header::CONTENT_TYPE, "application/json") 
            .json(&json)
            .send()
            .await?;

    // Check the response body
    let body = response.text().await?;
    println!("Response Body: {:?}", body);    
    
    // Deserialize JSON into struct
    let result: Result<ListInvoicesExt, serde_json::Error> = serde_json::from_str(&body);

    match result {
        Ok(your_struct) => {
            println!("Deserialized struct: {:?}", your_struct);
            Ok(your_struct)    
        }
        Err(e) => {
            println!("Error deserialized: {:?}", e);
            Err(CustomError::new(997, e.to_string()))
        }
    }

}    