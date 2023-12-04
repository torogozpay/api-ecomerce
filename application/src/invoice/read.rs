// application/src/invoice/read.rs

use lazy_static::lazy_static;

use domain::models::{BusinessNode,Invoice, InvoiceDet, MyInvoice};
use domain::modelsext::InvoiceResponse;
use infrastructure as db;
use diesel::prelude::*;
use shared::error_handler::CustomError;
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

    let mut conn = db::connection()?;

    let mut invoices = invoices::table.filter(invoices::business_id.eq(_config.business_id)).select(Invoice::as_select()).load(&mut conn)?;
    invoices.sort();

    Ok(invoices)
}


pub async fn list_invoice_by_hash(config : &BusinessNode, payment_hash : String) -> Result<InvoiceResponse, CustomError> {
    let api_key = CONFIG.api.api_key.clone();
    let username = CONFIG.api.api_username.clone();
    let password = CONFIG.api.api_password.clone();
    let auth_string = format!("{}:{}", username, password);
    let auth_encoded = "Basic ".to_owned() + &base64::encode(&auth_string);

    let socket: String;
    socket = config.host.to_string() + ":" + &config.port.to_string();

    let json = &serde_json::json!({
        "lnd": config.lnd,
        "socket": socket.to_string(),
        "macaroon": config.macaroon.to_string(),
        "cert": config.cert.to_string(),
        "path": config.path.to_string(),
        "hash": payment_hash.to_string()
    });

    // Construct the request
    let client = Client::builder().build()?; 
    let response = client
            .get("https://a085-138-186-251-29.ngrok-free.app/api/v1/getInvoice")
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
    let result: Result<InvoiceResponse, serde_json::Error> = serde_json::from_str(&body);

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