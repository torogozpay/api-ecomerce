// application/src/invoice/read.rs
use domain::modelsext::{Invoice,DataLookupInvoice};
use shared::error_handler::CustomError;
use reqwest::{header, Client};
use actix_web::http::header::map::HeaderMap;
use uuid::Uuid;

use shared::settings::CONFIG;
use tracing::{info, error};

pub async fn list_invoice_by_hash(headers: &HeaderMap, payment_hash : String) -> Result<DataLookupInvoice, CustomError> {
    // Check the presence of payment_hash
    if payment_hash.clone().trim().len() == 0 {
        error!("Error, payment_hash is required and must not be empty");
        return Err(CustomError::new(400, "payment_hash is required and must not be empty".to_string()));
    }
   
    let auth_encoded = headers.get("Authorization").expect("Error token").to_str().unwrap_or_default().to_string();

    let socket: String;
    socket = CONFIG.server.host.to_string() + ":8181";

    let json = &serde_json::json!({
        "hash": payment_hash.to_string()
    });

    // Construct the request
    let client = Client::builder().build()?; 
    let response = client
            .post("http://".to_owned() + &socket.to_string() + "/api/lightning/v1/lookupInvoice")
            .header("Authorization", &auth_encoded)
            .header(header::CONTENT_TYPE, "application/json") 
            .json(&json)
            .send()
            .await?;

    // Check the response body
    let body = response.text().await?;
    info!("Response Body: {:?}", body);    
    
    // Deserialize JSON into struct
    let result: Result<DataLookupInvoice, serde_json::Error> = serde_json::from_str(&body);

    match result {
        Ok(your_struct) => {
            info!("Deserialized Invoice: {:?}", your_struct);
            Ok(your_struct)    
        }
        Err(e) => {
            error!("Error deserialized: {:?}", e);
            Err(CustomError::new(997, e.to_string()))
        }
    }

}    

pub async fn list_invoice_by_uuid(headers: &HeaderMap, uuid : Uuid) -> Result<Invoice, CustomError> {
    // Check the presence of uuid
    if uuid.to_string().clone().trim().len() == 0 {
        error!("Error, uuid is required and must not be empty");
        return Err(CustomError::new(400, "uuid is required and must not be empty".to_string()));
    }

    let auth_encoded = headers.get("Authorization").expect("Error token").to_str().unwrap_or_default().to_string();

    let socket: String;
    socket = CONFIG.server.host.to_string() + ":8181";

    let json = &serde_json::json!({
        "uuid": uuid.to_string()
    });

    // Construct the request
    let client = Client::builder().build()?; 
    let response = client
            .post("http://".to_owned() + &socket.to_string() + "/api/lightning/v1/lookupOrder")
            .header("Authorization", &auth_encoded)
            .header(header::CONTENT_TYPE, "application/json") 
            .json(&json)
            .send()
            .await?;

    // Check the response body
    let body = response.text().await?;
    info!("Response Body: {:?}", body);    
    
    // Deserialize JSON into struct
    let result: Result<Invoice, serde_json::Error> = serde_json::from_str(&body);

    match result {
        Ok(your_struct) => {
            info!("Deserialized Invoice: {:?}", your_struct);
            Ok(your_struct)    
        }
        Err(e) => {
           error!("Error deserialized: {:?}", e);
            Err(CustomError::new(997, e.to_string()))
        }
    }

}    