// application/src/invoice/read.rs
use domain::modelsext::InvoiceResponse;
use shared::error_handler::CustomError;
use reqwest::{header, Client};

use shared::settings::CONFIG;

pub async fn list_invoice_by_hash(payment_hash : String) -> Result<InvoiceResponse, CustomError> {
    let api_key = CONFIG.api.api_key.clone();
    let username = CONFIG.api.api_username.clone();
    let password = CONFIG.api.api_password.clone();
    let auth_string = format!("{}:{}", username, password);
    let auth_encoded = "Basic ".to_owned() + &base64::encode(&auth_string);


    let socket: String;
    socket = CONFIG.api.api_server.to_string() + ":8181";

    let json = &serde_json::json!({
        "hash": payment_hash.to_string()
    });

    // Construct the request
    let client = Client::builder().build()?; 
    let response = client
            .get("http://".to_owned() + &socket.to_string() + "/api/v1/getInvoice")
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