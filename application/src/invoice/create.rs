// apllication/src/invoice/create.rs

use domain::models::{BusinessNode, Invoice, MyNewInvoice, MyInvoice, InvoiceDet, NewInvoice, NewInvoiceDet};
use infrastructure as db;
use diesel::prelude::*;
use shared::error_handler::CustomError;
//use anyhow::anyhow;
use reqwest::{header, header::HeaderMap,Client,Response};
//use cln_rpc::model::responses::InvoiceResponse;
use super::config_node;
use bigdecimal::BigDecimal;
use dotenv::dotenv;
use std::env;

//use prost::Message; // for deserializing gRPC messages
//use serde_json::to_string_pretty; // for serializing to JSON


pub async fn start_payment(mut myinvoice : MyNewInvoice) -> Result<MyInvoice, CustomError> {
   let config = config_node(&myinvoice.api_secret).await?;
   let _invoice_node: Response = generate_invoice(&config, &myinvoice.master.amount, &myinvoice.master.description).await?;

   myinvoice.master.business_id = config.business_id.clone();
/* 
    myinvoice.master.bolt11 = _invoice_node.bolt11;
    myinvoice.master.payment_hash = _invoice_node.payment_hash.to_string();
    //myinvoice.master.payment_secret = _invoice_node.payment_secret;
    myinvoice.master.expires_at = _invoice_node.expires_at.into();
    myinvoice.master.warning_capacity = _invoice_node.warning_capacity;
    myinvoice.master.warning_offline = _invoice_node.warning_offline;
    myinvoice.master.warning_deadends = _invoice_node.warning_deadends;
    myinvoice.master.warning_private_unused = _invoice_node.warning_private_unused;
    myinvoice.master.warning_mpp = _invoice_node.warning_mpp;
    myinvoice.master.payment_status = "Created".to_owned();
*/
    let invoice_api = create_invoice(myinvoice).await?;

    Ok(invoice_api)
}


pub async fn generate_invoice(config : &BusinessNode, amount : &BigDecimal, description : &String) -> Result<Response, CustomError> {
    let client = Client::builder().build()?; 

    // Load environment variables from the .env file
    dotenv().ok();

   // Encode your username and password as a Base64 string
   let username = env::var("USERNAME").expect("USERNAME not found in .env file");
   let password = env::var("PASSWORD").expect("PASSWORD not found in .env file");
   let auth_string = format!("{}:{}", username, password);
   let encoded_auth = base64::encode(&auth_string);
   // Retrieve the API key from the environment variable
    let api_key = env::var("API_KEY").expect("API_KEY not found in .env file");

        
    let query = vec![
        //("path", config.path.to_string()),   
        ("expiry", config.expiry.to_string()),   
        ("cltv", config.cltv.to_string()),   
        ("amount", amount.to_string()),    
        ("description", description.to_string()),      
    ];


    // Construct the request
    let response = client       
            .get("https://64c7-138-186-251-29.ngrok-free.app/api/v1/createInvoice")
            .header(header::AUTHORIZATION, format!("Basic {}", encoded_auth))
            .header("x-api-key", api_key) 
            //.header(header::AUTHORIZATION, format!("Bearer {}", token)) 
            //.header(header::CONTENT_TYPE, "application/json") 
            //.query(&query) 
            .json(&serde_json::json!(query))
            .send()
            .await?;

    // Parse and process the response
    if response.status().is_success() {
        // Serialize the gRPC response struct to JSON
        //let json_response = to_string_pretty(&response).unwrap();
        //println!("{:?}", json_response);
        println!("{:?}", response);
    } else {
        println!("Request failed with status: {}", response.status());
    }

    Ok(response)    
}    


pub async fn create_invoice(myinvoice: MyNewInvoice) -> Result<MyInvoice, CustomError> { 
    use domain::schema::invoices;
    use domain::schema::invoices_det;

    let mut conn = db::connection()?;

    let invoice = NewInvoice::from(myinvoice.master);

    let newinvoice = diesel::insert_into(invoices::table).values(&invoice).get_result::<Invoice>(&mut conn)?;  

    let mut invoicedets: Vec<NewInvoiceDet> = Vec::new();
    for mut element in myinvoice.details {
        element.invoice_id = newinvoice.id;
        let row = NewInvoiceDet::from(element);
        invoicedets.push(row);
    } 

    let newinvoicedets = diesel::insert_into(invoices_det::table).values(&invoicedets).get_results::<InvoiceDet>(&mut conn)?;

    let myresult = MyInvoice { api_secret: (myinvoice.api_secret), master: (newinvoice), details: (newinvoicedets) };
    Ok(myresult)
}