// application/src/invoice/create.rs
#![allow(non_snake_case)]

use domain::models::Currencies;
use domain::modelsext::{NewInvoice, NewInvoiceDet, InvoiceData, InvoiceResponse, Preorder};
use crate::business;
use infrastructure as db;
use diesel::prelude::*;
use shared::settings::CONFIG;
use shared::error_handler::CustomError;
use reqwest::{header, Client};

use crate::bitcoin as priceBitcoin;
use bigdecimal::{BigDecimal,ToPrimitive,Zero};
//use chrono::Utc;
use tracing::info;

pub async fn save_pre_order(business_id: i32, mut preventa: NewInvoice) -> Result<InvoiceResponse, CustomError> {
    let mybusiness = business::read::list_business(business_id);

    preventa.businessId = business_id; 
    preventa.applySplit = mybusiness.unwrap().apply_split; 

    // Verificar la presencia de business_id
    if preventa.businessId <= 0 {
       return Err(CustomError::new(101, "business_id is required".to_string()));
    }
    // Verificar la presencia de order_id
    if preventa.orderId <= 0 {
       return Err(CustomError::new(102, "order_id is required".to_string()));
    }

    // Verificar la presencia de customer_name
    if preventa.customerName.trim().len() == 0 {
       return Err(CustomError::new(103, "customer_name is required and must not be empty".to_string()));
    }

    // Verificar la presencia de customer_email
    if preventa.customerEmail.trim().len() == 0 {
       return Err(CustomError::new(104, "customer_email is required and must not be empty".to_string()));
    }

    // Verificar la presencia de currency
    if preventa.currency.trim().len() == 0 {
       return Err(CustomError::new(105, "currency is required and must not be empty".to_string()));
    }

    // Verificar la presencia de sub_total
    if preventa.subTotal <= BigDecimal::zero() {
       return Err(CustomError::new(106, "sub_total is required and must not be zero".to_string()));
    }
/*
    // Verificar la presencia de taxes
    if preventa.taxes <= BigDecimal::zero() {
       return Err(CustomError::new(107, "taxes is required and must not be zero".to_string()));
    }
*/
    // Verificar la presencia de total_amount
    if preventa.totalAmount <= BigDecimal::zero() {
       return Err(CustomError::new(108, "total_amount is required and must not be zero".to_string()));
    }
/*
    // Verificar la presencia de invoice_date
    if preventa.invoiceDate < Utc::now() {
       return Err(CustomError::new(109, "invoice_date is required and must not be less than today".to_string()));
    }
*/
    // Verificar la presencia de invoice_stamp
    if preventa.invoiceStamp.trim().len() == 0  {
       return Err(CustomError::new(110, "invoice_stamp is required".to_string()));
    }

    // Verificar la presencia de details
    if !preventa.details.iter().all(is_valid_row_details) {
        return Err(CustomError::new(111, "details is required".to_string()));
    }

    //let preorder = paso1_api_business_pre_order(preventa.clone()).await?;
    let preorder = Preorder {
        presell_id: preventa.orderId,
        split_id: preventa.orderId
    };
    let ln = paso2_api_lightning_invoice(preorder, preventa.clone()).await?;

    info!("business_id {:?}",ln.business_id);

    Ok(ln)
}  

fn is_valid_row_details(det: &NewInvoiceDet) -> bool {
    let _cond1 = det.productId > 0;
    let _cond2 = det.productName.trim().len() > 0;
    let _cond3 = det.quantity > 0;
    let _cond4 = det.subTotal > BigDecimal::zero();
    let _cond5 = true; //det.taxes > BigDecimal::zero();
    let _cond6 = det.grandTotal > BigDecimal::zero();

    _cond1 & _cond2 & _cond3 & _cond4 & _cond5 & _cond6
} 

pub async fn paso1_api_business_pre_order(preventa: NewInvoice) -> Result<Preorder, CustomError> {
    let api_key = CONFIG.api.api_key.clone();
    let username = CONFIG.api.api_username.clone();
    let password = CONFIG.api.api_password.clone();
    let auth_string = format!("{}:{}", username, password);
    let auth_encoded = "Basic ".to_owned() + &base64::encode(&auth_string);
    
    let socket: String;
    socket = CONFIG.server.host.to_string() + ":8383";
    
    // Construct the request
    let client = Client::builder().build()?; 
    let response = client
            .post("http://".to_owned() + &socket.to_string() + "/api/v1/savePreorder")
            .header("Authorization", format!("{}", auth_encoded))
            .header("x-api-key", api_key) 
            .header(header::CONTENT_TYPE, "application/json") 
            .json(&preventa)
            .send()
            .await?;

    // Check the response body
    let body = response.text().await?;
    info!("Response Body: {:?}", body);
    
    // Deserialize JSON into struct
    let result: Result<Preorder, serde_json::Error> = serde_json::from_str(&body);

    match result {
        Ok(your_struct) => {
            info!("Deserialized struct: {:?}", your_struct);
            Ok(your_struct)    
        }
        Err(e) => {
            info!("Error deserialized: {:?}", e);
            Err(CustomError::new(997, e.to_string()))
        }
    }
}    


pub async fn paso2_api_lightning_invoice(preorder: Preorder, preventa: NewInvoice) -> Result<InvoiceResponse, CustomError> {
    let api_key = CONFIG.api.api_key.clone();
    let username = CONFIG.api.api_username.clone();
    let password = CONFIG.api.api_password.clone();
    let auth_string = format!("{}:{}", username, password);
    let auth_encoded = "Basic ".to_owned() + &base64::encode(&auth_string);
    
    let socket: String;
    socket = CONFIG.api.api_server.to_string() + ":8181";

    
    use domain::schema::currencies;

    let mut conn = db::connection()?;
    
    let curr = currencies::table.filter(currencies::currency.eq(preventa.currency.clone())).select(Currencies::as_select()).get_result(&mut conn)?;
    let amount = priceBitcoin::convert_currency_to_satoshi(curr, preventa.totalAmount.to_f64().unwrap());
 
    match amount.await {
        Ok(amount) => {
            let description1 = "BusinessId: ".to_string() + &preventa.businessId.to_string();
            let description2 = "/ OrderId: ".to_owned() + &preventa.orderId.to_string();
            let description3 =  "/ Currency: ".to_owned() + &preventa.currency.to_string();
            let description4 =  "/ Amount: ".to_owned() + &preventa.totalAmount.to_string();
            let description = description1 + &description2 + &description3 + &description4; 

            let invoice = InvoiceData {
                business_id: preventa.businessId,
                order_id: preventa.orderId,
                presell_id: preorder.presell_id,
                split_id: preorder.split_id,
                name: preventa.customerName,
                email: preventa.customerEmail,
                invoice_date: preventa.invoiceDate,
                description: description,
                currency: preventa.currency,
                total_amount: preventa.totalAmount.clone(),
                amount_msat: amount as i64,
                apply_split: preventa.applySplit
            };    

            // Construct the request
            let client = Client::builder().build()?; 
            let response = client
                    .post("http://".to_owned() + &socket.to_string() + "/api/v1/createInvoice")
                    .header("Authorization", format!("{}", auth_encoded))
                    .header("x-api-key", api_key) 
                    .header(header::CONTENT_TYPE, "application/json") 
                    .json(&invoice)
                    .send()
                    .await?;

            // Check the response body
            let body = response.text().await?;
            info!("Response Body: {:?}", body);

            // Deserialize JSON into struct
            let result: Result<InvoiceResponse, serde_json::Error> = serde_json::from_str(&body);

            match result {
                Ok(your_struct) => {
                    info!("Deserialized struct: {:?}", your_struct);
                    return Ok(your_struct);    
                }
                Err(e) => {
                    info!("Error deserialized: {:?}", e);
                    return Err(CustomError::new(997, e.to_string()));
                }
            }
        } 
        Err(_) => {
            Err(CustomError::new(990, "Invalid convertion to satoshis".to_string()))
        }    
    }    
}